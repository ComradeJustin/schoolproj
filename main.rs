use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect}, prelude::{CrosstermBackend, Stylize, Terminal}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph, Widget}
};
use std::{default, io::{stdout, Result}, time::Duration};



fn main() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?; // this section sets up the start of the program
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?; // disables all input
    terminal.clear()?; //clears everything
    
    let _ = mainloop(); // Runs main loop of rendering
    stdout().execute(LeaveAlternateScreen)?; //starts ending the program
    disable_raw_mode()?; //Reactivates all inputs


    Ok(())
}

fn mainloop()-> Result<()> {
    let mut inputval = "".to_string();


    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(loop {
       
    let _ = terminal.draw(|f|{
        let lay = Rect::new(0,0, f.size().width,f.size().height);
        let lout = Layout::default().direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100 / 2),
            Constraint::Percentage(100 / 2)
        ])
        .split(lay);

      let lside = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(90),
                Constraint::Percentage(100 - 90),




        ]
    ).split(lout[0]);


    let rside = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(100),
                



        ]
    ).split(lout[1]);


       f.render_widget(Paragraph::new(format!(
        "Welcome to the game.\n\
        This is a test messsage.
        "
        )).style(
            Style::default()
            .fg(Color::Blue))
            .block(Block::default()
            .title("Todo game")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(Style::default()
            .fg(Color::Green))),
             lside[0]);


        
        f.render_widget(Paragraph::new(format!(
                "{inputval}
                "
                )).style(
                    Style::default()
                    .fg(Color::Blue))
                    .block(Block::default()
                    .title("input")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(Style::default()
                    .fg(Color::Green))),
                     lside[1]);







        f.render_widget(Paragraph::new(format!(
                        "Welcome to the game.\n\
                        This is a test messsage.
                        "
                        )).style(
                            Style::default()
                            .fg(Color::Blue))
                            .block(Block::default()
                            .title("test")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .style(Style::default()
                            .fg(Color::Green))),
                             rside[0]);
    });
    
  


    if crossterm::event::poll(Duration::from_millis(250))?{
        if let Event::Key(key) = crossterm::event::read()?{
            if key.code == KeyCode::Esc{
                break;
            }
            if key.code == KeyCode::Enter{
                //Input function here
            }
            if key.kind == KeyEventKind::Release{
                match key.code{
                    KeyCode::Char(value) => {
                        inputval.push(value)
                    },
                    KeyCode::Backspace =>{
                        if inputval.len() > 0{
                            inputval.truncate(inputval.len() - 1)
                        }
                        
                    }
                    _ => {}
                }

            }
            
        }

    }
        




    })


    
}



//todo