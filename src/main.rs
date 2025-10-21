use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use iced::{
    widget::{checkbox, column, container, text, Image},
    Application, Command, Element, Length, Settings, Subscription, Theme,
};
use image::{imageops, DynamicImage, ImageFormat};
use reqwest;
use std::io::Cursor;
use std::time::Duration;

fn main() -> iced::Result {
    let settings = Settings {
        id: Some("xcoffee".to_string()),
        ..Settings::default()
    };
    XCoffee::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    ImageLoaded(Result<Vec<u8>, String>),
    Error(String),
    ToggleTrojanView(bool),
}

struct XCoffee {
    image_data: Option<Vec<u8>>,
    status: String,
    loading: bool,
    trojan_view: bool,
}

impl Application for XCoffee {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                image_data: None,
                status: "Connecting to stream...".to_string(),
                loading: true,
                trojan_view: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("xcoffee")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ImageLoaded(Ok(data)) => {
                self.image_data = Some(data);
                self.status = String::new();
                self.loading = false;
            }
            Message::ImageLoaded(Err(e)) => {
                self.status = format!("Frame error: {}", e);
                self.loading = false;
            }
            Message::Error(e) => {
                self.status = format!("Stream error: {}", e);
                self.loading = false;
            }
            Message::ToggleTrojanView(enabled) => {
                self.trojan_view = enabled;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let image_widget = if let Some(data) = &self.image_data {
            let processed_data = if self.trojan_view {
                apply_trojan_filter(data).unwrap_or_else(|_| data.clone())
            } else {
                data.clone()
            };

            container(
                Image::new(iced::widget::image::Handle::from_memory(processed_data))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
        } else {
            container(
                text(if self.loading {
                    "Loading coffee pot image..."
                } else {
                    "No image available"
                })
                .size(14),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
        };

        let checkbox_widget = container(
            checkbox("Trojan View", self.trojan_view)
                .on_toggle(Message::ToggleTrojanView)
                .size(16)
                .text_size(14),
        )
        .padding(10)
        .width(Length::Fill)
        .center_x();

        column![image_widget, checkbox_widget]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        enum State {
            Connecting,
            Streaming {
                stream: Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send + Unpin>,
                buffer: Vec<u8>,
                boundary: Vec<u8>,
                is_first_frame: bool,
            },
            Sleeping(Duration),
        }

        struct MjpegSub;
        const URL: &str = "https://kaffee.hnf.de";

        iced::subscription::unfold(
            std::any::TypeId::of::<MjpegSub>(),
            State::Connecting,
            move |state| async move {
                match state {
                    State::Connecting => match reqwest::get(URL).await {
                        Ok(response) => {
                            if response.status().is_success() {
                                let content_type = response
                                    .headers()
                                    .get("content-type")
                                    .and_then(|value| value.to_str().ok());

                                if let Some(ct) = content_type {
                                    if let Some(boundary_str) =
                                        ct.split(';').find(|s| s.trim().starts_with("boundary="))
                                    {
                                        let boundary =
                                            boundary_str.split('=').nth(1).unwrap_or("").trim();
                                        if boundary.is_empty() {
                                            (
                                                Message::Error(
                                                    "Empty boundary in Content-Type header"
                                                        .to_string(),
                                                ),
                                                State::Sleeping(Duration::from_secs(5)),
                                            )
                                        } else {
                                            let full_boundary =
                                                format!("--{}", boundary).into_bytes();
                                            (
                                                Message::Error(
                                                    "Connected. Waiting for frame...".to_string(),
                                                ),
                                                State::Streaming {
                                                    stream: Box::new(response.bytes_stream()),
                                                    buffer: Vec::new(),
                                                    boundary: full_boundary,
                                                    is_first_frame: true,
                                                },
                                            )
                                        }
                                    } else {
                                        (
                                            Message::Error(
                                                "Boundary not found in Content-Type header"
                                                    .to_string(),
                                            ),
                                            State::Sleeping(Duration::from_secs(5)),
                                        )
                                    }
                                } else {
                                    (
                                        Message::Error("Missing Content-Type header".to_string()),
                                        State::Sleeping(Duration::from_secs(5)),
                                    )
                                }
                            } else {
                                (
                                    Message::Error(format!(
                                        "Connection failed with status: {}",
                                        response.status()
                                    )),
                                    State::Sleeping(Duration::from_secs(5)),
                                )
                            }
                        }
                        Err(e) => (
                            Message::Error(format!("Connection error: {}", e)),
                            State::Sleeping(Duration::from_secs(5)),
                        ),
                    },
                    State::Streaming {
                        mut stream,
                        mut buffer,
                        boundary,
                        is_first_frame,
                    } => loop {
                        let boundary_to_search = if is_first_frame {
                            boundary.clone()
                        } else {
                            [b"\r\n", boundary.as_slice()].concat()
                        };

                        if let Some(boundary_pos) = buffer
                            .windows(boundary_to_search.len())
                            .position(|w| w == &boundary_to_search)
                        {
                            let part_data = &buffer[..boundary_pos];
                            if !part_data.is_empty() {
                                let header_body_separator = b"\r\n\r\n";
                                if let Some(separator_pos) = part_data
                                    .windows(header_body_separator.len())
                                    .position(|w| w == header_body_separator)
                                {
                                    let image_data = part_data
                                        [separator_pos + header_body_separator.len()..]
                                        .to_vec();
                                    if !image_data.is_empty() {
                                        buffer.drain(..boundary_pos + boundary_to_search.len());
                                        break (
                                            Message::ImageLoaded(Ok(image_data)),
                                            State::Streaming {
                                                stream,
                                                buffer,
                                                boundary,
                                                is_first_frame: false,
                                            },
                                        );
                                    }
                                }
                            }
                            buffer.drain(..boundary_pos + boundary_to_search.len());
                        } else {
                            match stream.next().await {
                                Some(Ok(chunk)) => {
                                    buffer.extend_from_slice(&chunk);
                                }
                                Some(Err(e)) => {
                                    break (
                                        Message::Error(format!("Stream error: {}", e)),
                                        State::Sleeping(Duration::from_secs(5)),
                                    );
                                }
                                None => {
                                    break (
                                        Message::Error("Stream ended. Reconnecting...".to_string()),
                                        State::Sleeping(Duration::from_secs(5)),
                                    );
                                }
                            }
                        }
                    },
                    State::Sleeping(duration) => {
                        // Reconnect after a delay
                        tokio::time::sleep(duration).await;
                        (
                            Message::Error("Reconnecting...".to_string()),
                            State::Connecting,
                        )
                    }
                }
            },
        )
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn apply_trojan_filter(image_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Load the image
    let img = image::load_from_memory(image_data)?;

    // Convert to grayscale
    let gray_img = imageops::grayscale(&img);

    // Calculate dimensions to fit within 128x128 while preserving aspect ratio
    let (orig_width, orig_height) = gray_img.dimensions();
    let aspect_ratio = orig_width as f32 / orig_height as f32;

    let (new_width, new_height) = if aspect_ratio > 1.0 {
        // Width is larger, scale to 128 width
        (128, (128.0 / aspect_ratio) as u32)
    } else {
        // Height is larger or equal, scale to 128 height
        ((128.0 * aspect_ratio) as u32, 128)
    };

    // Resize using nearest neighbor for pixelated effect
    let resized = imageops::resize(
        &gray_img,
        new_width,
        new_height,
        imageops::FilterType::Nearest,
    );

    // Convert back to DynamicImage
    let small_img = DynamicImage::ImageLuma8(resized);

    // Encode to JPEG with low quality for vintage camera effect
    let mut output = Cursor::new(Vec::new());
    small_img.write_to(&mut output, ImageFormat::Jpeg)?;

    Ok(output.into_inner())
}
