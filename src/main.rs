use iced::{executor, Application, Command, Element, Text, Settings,
button, Align, Button, Column, Font, HorizontalAlignment, Length, Row, Subscription};

struct GUI{
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI{
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (GUI, Command<Self::Message>){
        (
            GUI{
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none())
    }

    fn title(&self) -> String{
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message>{
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message>{
        let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
            .min_width(80);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
            .min_width(80);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()

    }
}

fn main() {
    GUI::run(Settings::default());
}
