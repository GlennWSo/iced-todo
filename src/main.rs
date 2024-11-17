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
    theme: Theme,
    input_value: String,
    todo: HashMap<Uuid, Task>,
    finnished: HashMap<Uuid, Task>,
}

#[derive(Clone, Debug)]
enum Message {
    Input(String),
    CreateTask,
    CompleteTask(Uuid),
    RedoTask(Uuid),
    DeleteTask(Uuid),
}

impl State {
    fn view(&self) -> Element<Message> {
        let new_todo = row![
            button("add todo").on_press(Message::CreateTask),
            text_input("Do this", &self.input_value).on_input(|input| Message::Input(input))
        ];
        let todos = column(self.todo.values().map(|task| {
            let tx: Element<Message> = button(task.description.as_str())
                .on_press(Message::CompleteTask(task.id))
                .into();
            tx
        }));
        let finnished = column(self.finnished.values().map(|task| {
            let tx: Element<Message> = button(task.description.as_str())
                .style(|theme, status| button::secondary(theme, status))
                .on_press(Message::RedoTask(task.id))
                .into();
            tx
        }));

        let main_box = column![new_todo, text("Do"), todos, text("Done"), finnished].spacing(10);
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
            Message::CompleteTask(id) => {
                let fin = self.todo.remove(&id);
                if let Some(task) = fin {
                    self.finnished.insert(id, task);
                };
            }
            Message::RedoTask(id) => {
                let fin = self.finnished.remove(&id);
                if let Some(task) = fin {
                    self.todo.insert(id, task);
                };
            }
            Message::DeleteTask(_) => todo!(),
        }
    }
}

fn main() -> std::result::Result<(), iced::Error> {
    iced::run("Todo", State::update, State::view)
}
