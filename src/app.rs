use ratatui::text::Line;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// text input
    pub chat_input: String,
    /// how much to scroll text
    pub chat_scroll: (u16, u16),
    /// the text of the chat
    pub chat_text: Vec<Line<'a>>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            chat_input: String::new(),
            chat_scroll: (0, 0),
            chat_text: vec![Line::raw("Lorem ipsum dolor sit amet, consectetur adipiscing elit."), Line::raw("Nulla at ante imperdiet, laoreet nunc eget, tincidunt dui."), Line::raw("Maecenas volutpat at est eu varius."), Line::raw("Aliquam rhoncus tempus lectus, et finibus est laoreet eu."), Line::raw("Donec rhoncus tempus consectetur."), Line::raw("Curabitur molestie venenatis velit vel sodales."), Line::raw("Proin eget tristique felis."), Line::raw("Aenean venenatis lacus vel magna efficitur, id euismod massa interdum."), Line::raw("Curabitur dolor nibh, mollis nec euismod quis, lacinia quis turpis."), Line::raw("Sed egestas turpis eget nunc accumsan, a finibus purus malesuada."), Line::raw("Sed rhoncus, est ut vehicula condimentum, ipsum neque condimentum justo, et commodo arcu velit ac orci."), Line::raw("Nullam at pretium massa."), Line::raw("Curabitur non eros at mauris pretium vestibulum non ut diam."), Line::raw("In ullamcorper massa nec velit venenatis ullamcorper."), Line::raw("Curabitur pretium malesuada dolor vel convallis."), Line::raw("Maecenas iaculis sollicitudin urna, ut fringilla mauris scelerisque vitae."), Line::raw("Phasellus scelerisque quam at ornare sollicitudin."), Line::raw("Sed in nibh velit."), Line::raw("Morbi malesuada ullamcorper nisi sollicitudin facilisis."), Line::raw("Vestibulum posuere, ex eu sagittis eleifend, ipsum ipsum rhoncus lorem, at luctus felis risus vel lacus."), Line::raw("Integer rutrum, est vel accumsan fringilla, ex erat ullamcorper ex, ut consectetur tortor mi a tortor."), Line::raw("Integer elementum eu magna nec placerat."), Line::raw("Nulla commodo commodo tempus."), Line::raw("Ut sed tortor maximus, aliquet dui non, consectetur nisl."), Line::raw("Nunc ac turpis rutrum, elementum lacus sed, hendrerit libero."), Line::raw("Vivamus congue leo imperdiet purus tempor, eu vestibulum mi ultricies."), Line::raw("Cras fermentum metus justo, venenatis hendrerit metus auctor eget."), Line::raw("Nam facilisis justo est, eget faucibus mauris convallis at."), Line::raw("Sed pulvinar, nibh a mattis congue, odio enim commodo turpis, non molestie odio odio nec ex."), Line::raw("Integer bibendum, nisl id dignissim dignissim, risus risus auctor ante, eu rhoncus ante odio non enim."), Line::raw("Aliquam commodo nibh et neque auctor auctor."), Line::raw("Nullam sit amet ipsum felis."), Line::raw("Nam sit amet massa mollis, gravida mauris eget, sollicitudin massa."), Line::raw("Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas."), Line::raw("Fusce interdum luctus elit, id molestie lectus hendrerit in."), Line::raw("Curabitur aliquam, libero vel hendrerit ornare, felis risus mollis elit, at fermentum odio massa eleifend diam."), Line::raw("In tempor, lorem at scelerisque scelerisque, augue augue viverra augue, eget sodales enim ligula sit amet elit."), Line::raw("Sed dui augue, porttitor ac erat non, laoreet mollis turpis."), Line::raw("Mauris non purus lectus."), Line::raw("Proin eleifend ut quam sed dapibus."), Line::raw("Aenean dapibus odio eget urna maximus sagittis."), Line::raw("Etiam feugiat dignissim iaculis."), Line::raw("Praesent arcu ligula, commodo accumsan ultrices in, facilisis at ex."), Line::raw("Duis vitae neque ornare, imperdiet purus nec, dapibus nulla."), Line::raw("Vivamus at gravida neque, vitae molestie ex."), Line::raw("Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos."), Line::raw("Sed vitae eleifend eros."), Line::raw("Interdum et malesuada fames ac ante ipsum primis in faucibus."), Line::raw("Vestibulum venenatis, dolor at fringilla pretium, lorem nisl egestas tortor, feugiat feugiat lorem velit et dolor."), Line::raw("Donec sed magna sit amet odio pulvinar tempor eget sed lorem."), Line::raw("Fusce feugiat, lectus rutrum porttitor ultricies, ex purus sagittis libero, in porta ipsum odio nec urna."), Line::raw("Curabitur maximus fringilla purus in suscipit."), Line::raw("Ut feugiat aliquam condimentum.")],
        }
    }
}

impl App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn scroll_down(&mut self) {
        if self.chat_scroll.0 < self.chat_text.len() as u16 - 1 {
            self.chat_scroll.0 += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.chat_scroll.0 > 0 {
            self.chat_scroll.0 -= 1;
        }
    }
}
