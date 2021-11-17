use iced::{slider, Column, Element, Sandbox, Settings, Slider, Text};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    input: slider::State,
    value_kg: f32,
    old: slider::State,
    text: String,
    value_m: f32,
    typ: String,
}

#[derive(Debug, Clone)]
enum Message {
    Change(f32),
    Update(f32),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Change(f32) => {
                self.value_kg = f32;
                self.text = format!("{}", (self.value_kg)/((self.value_m*self.value_m)/10000.));
                // println!("{}", f32);
                self.typ = matchthat(self.text.clone());
            }
            Message::Update(value_m) => {
                self.value_m = value_m;
                self.text = format!("{}", (self.value_kg)/((self.value_m*self.value_m)/10000.));
                // println!("{}", value_m);
                self.typ = matchthat(self.text.clone());
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(200)
            .align_items(iced::Align::Center)
            .push(Text::new(format!("Ile kg ważysz? {}kg",self.value_kg )).size(40))
            .push(
                Slider::new(&mut self.old, 0.0..=100.0, self.value_kg, Message::Change).step(0.01),
            )
            .push(Text::new(format!("Ile masz wzrostu? {}cm", self.value_m)).size(40))
            .push(
                Slider::new(&mut self.input, 0.0..=200.0, self.value_m, Message::Update).step(0.01),
            )
            .push(Text::new(format!("Twoje BMI wynosi {}", self.text)).size(40))
            .push(Text::new(format!("{}", self.typ)).size(40))
            .into()
    }
}
fn matchthat(f: String) -> String {
    let f = f.parse::<f32>().unwrap();
    if f < 18.5 {
        String::from("Niedowaga")
    } else if f < 25.0 {
        String::from("Waga prawidłowa")
    } else if f < 30.0 {
        String::from("Nadwaga")
    } else {
        String::from("Otyłość")
    }
}