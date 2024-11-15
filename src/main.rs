use std::collections::HashMap;

use derivative::Derivative;
use iced::{
    // advanced::graphics::core::Element,
    advanced::Widget,
    border::width,
    futures::FutureExt,
    widget::{button, column, container, row, text, text_input},
    Element,
    Length::Fill,
    Theme,
};
use uuid::Uuid;

// #[derivative(Hash)]
// #[derive(Derivative)]
#[derive(Hash, Debug)]
struct Task {
    description: String,
    id: Uuid,
}

impl Task {
    fn new(description: String) -> Self {
        Self {
            description,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Default)]
struct State {
    input_value: String,
    todo: HashMap<Uuid, Task>,
    finnished: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Message {
    Input(String),
    CreateTask,
    TaskCompleted(Uuid),
    TaskRedo(usize),
    TodoDeleted(usize),
    FinnishedDeleted(usize),
}

impl State {
    fn view(&self) -> Element<Message> {
        let new_todo = row![
            button("add todo").on_press(Message::CreateTask),
            text_input("Do this", &self.input_value).on_input(|input| Message::Input(input))
        ];
        let todos = column(self.todo.values().map(|task| {
            let tx: Element<Message> = button(task.description.as_str())
                .on_press(Message::TaskCompleted(task.id))
                .into();
            tx
        }));
        let main_box = column![new_todo, todos,];
        main_box.into()
    }
    fn update(&mut self, m: Message) {
        match m {
            Message::Input(txt) => self.input_value = txt,
            Message::CreateTask => {
                // self.todo.push(Task::new(self.input_value.clone()));
                let task = Task::new(self.input_value.clone());
                self.todo.insert(task.id, task);
            }
            Message::TaskCompleted(id) => todo!(),
            Message::TaskRedo(_) => todo!(),
            Message::TodoDeleted(_) => todo!(),
            Message::FinnishedDeleted(_) => todo!(),
        }
    }
}

fn main() -> std::result::Result<(), iced::Error> {
    iced::run("Todo", State::update, State::view)
}
