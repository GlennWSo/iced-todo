use iced::{
    border::width,
    widget::{button, column, container, text},
    Element,
    Length::Fill,
};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Clone, Debug, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
    fn view(&self) -> Element<Message> {
        container(
            column![
                button(text("+").center().width(Fill)).on_press(Message::Increment),
                // .height(Fill),
                text(self.value).center().width(Fill),
                button(text("-").center().width(Fill))
                    .on_press(Message::Decrement)
                    .clip(true)
            ]
            .spacing(11)
            .width(100),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}

fn main() -> iced::Result {
    iced::run("Lets count", Counter::update, Counter::view)
}
