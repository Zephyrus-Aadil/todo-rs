use pancurses as pc;
use crate::{Tab, CuiResponse, CuiState, CoreState};

impl Tab {
    fn toggle(&mut self) {
        *self = match *self {
            Self::Todo => Self::Done,
            Self::Done => Self::Todo,
        }
    }
}

impl CuiState<'_> {
    pub fn init(core_state: &CoreState) -> CuiState {
        let win = pc::initscr();
        pc::curs_set(0);
        CuiState {
            win,
            curr_tab: Tab::Todo,
            todo_curs: 0,
            done_curs: 0,
            core_state,
        }
    }

    pub fn end() {
        pc::endwin();
    }

    pub fn update(&mut self, key_input: Option<pc::Input>) -> CuiResponse {
        if let Some(key) = key_input {
            // `handle_input` returns false to exit the ui_loop
            if !self.handle_input(key) {
                return CuiResponse::Quit;
            }
        }

        self.render();

        CuiResponse::UserInput(
            self.win.getch()
        )
    }

    // Returns false to exit the ui_loop
    fn handle_input(&mut self, key: pc::Input) -> bool {
        match key {
            pc::Input::Character('q')  => return false,
            pc::Input::Character('\t') => self.curr_tab.toggle(),
            pc::Input::Character('k')  => self.cursor_up(),
            pc::Input::Character('j')  => self.cursor_down(),
            _ => (),
        }

        true
    }

    fn cursor_up(&mut self) {
        match self.curr_tab {
            Tab::Todo
                if self.todo_curs > 0 => self.todo_curs -= 1,
            Tab::Done
                if self.done_curs > 0 => self.done_curs -= 1,
            _ => (),
        }
    }

    fn cursor_down(&mut self) {
        match self.curr_tab {
            Tab::Todo
                if self.todo_curs < self.core_state.todo_list.len() - 1
                    => self.todo_curs += 1,
            Tab::Done
                if self.todo_curs < self.core_state.todo_list.len() - 1
                    => self.todo_curs += 1,
            _ => (),
        }
    }

    fn render(&self) {
        self.win.clear();
        self.win.printw("Simple Todo App:\n");

        match self.curr_tab {
            Tab::Todo => {
                self.win.printw("[ Todo ]  Done\n\n");
                self.render_list(&self.core_state.todo_list, self.todo_curs);
            }
            Tab::Done => {
                self.win.printw("  Todo  [ Done ]\n\n");
                self.render_list(&self.core_state.done_list, self.done_curs);
            }
        }

        self.win.refresh();
    }

    fn render_list(&self, list: &Vec<String>, cursor: usize) {
        for (i, element) in list.iter().enumerate() {
            if i == cursor {
                self.win.printw(format!("-> | {element}\n"));
            }
            else {
                self.win.printw(format!("  | {element}\n"));
            }
        }
    }
}

