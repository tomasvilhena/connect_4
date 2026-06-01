use std::io::stdout;

use colored::{self, Colorize};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};

use crate::Player::Player1;

#[derive(PartialEq, Clone, Copy, Debug)]
enum CellState
{
  Empty,
  Occupied(Player),
  HasWall,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Player
{
  Player1,
  Player2,
}

fn print_board(board: &Vec<Vec<CellState>>, selected: (usize, usize), current_player: Player)
{
  let mut is_line: bool;
  let mut is_first: bool = true;

  for y in 0..13
  {
    for x in 0..15
    {
      if board[y][x] == CellState::HasWall
      {
        if y % 2 == 0 && y != 0
        {
          if x == 0 || x % 2 == 0
          {
            is_line = true;
          } else
          {
            is_line = false;
          }

          if is_line
          {
            if is_first
            {
              print!("+");
              is_first = !is_first;
            }

            if x <= 13
            {
              if y == 12
              {
                print!("---");
              } else
              {
                print!("   ");
              }
            }
          } else
          {
            print!("+");
          }
          continue;
        } else if x % 2 == 0 && y != 0
        {
          print!("|");
        }
      } else if y == selected.1 && x == selected.0 && board[selected.0][selected.1] == CellState::Empty
      {
        if current_player == Player::Player1 
        {
          print!("{}", "▇▇▇".bright_green());
        } else if current_player == Player::Player2
        {
          print!("{}", "▇▇▇".bright_red());
        }
      } else if board[y][x] == CellState::Empty
      {
        print!("   ");
      } else if board[y][x] == CellState::Occupied(Player::Player1)
      {
        print!("{}", "▇▇▇".bright_yellow());
      } else if board[y][x] == CellState::Occupied(Player::Player2)
      {
        print!("{}", "▇▇▇".bright_blue());
      }
    }

    is_first = true;
    print!("\r\n");
  }
}

fn set_up_board(board: &mut Vec<Vec<CellState>>)
{
  for y in 0..13
  {
    for x in 0..15
    {
      if y == 0 || y % 2 == 0 || x == 0 || x % 2 == 0
      {
        board[y][x] = CellState::HasWall;
        continue
      }
    }
  }
}

fn check_for_win(board: &Vec<Vec<CellState>>) 
{
  //for every cell
  // you need to find all adjacent cells that could make a 4 in a row
  // for that to be possible oyud chekc all 4 cells above, 
  // all for 4 cells bellow
  // all cells above and below togheter becuas eit could be centered and then have one below and stuf like that
  // and all fot hat for diagonal 2
  // while going form odd to odd number
  // 
  // 
}

fn main()
{
  enable_raw_mode().unwrap();
  let mut board: Vec<Vec<CellState>> = vec![vec![CellState::Empty; 15]; 13];
  let mut selected = (1, 11);
  let mut current_player = Player::Player1;
  execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
  set_up_board(&mut board);
  print_board(&board, selected, current_player);

  'game_loop: loop
  {
    if let Event::Key(event) = read().unwrap()
    {
      match event.code
      {
        KeyCode::Left =>
        {
          
        },

        KeyCode::Right =>
        {

        },

        KeyCode::Enter =>
        {

        },

        KeyCode::Esc =>
        {
          std::process::exit(0);
        },

        _ => {},
      }
    }
  }
}
