use colored::{self, Colorize};

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

fn print_board(board: &Vec<Vec<CellState>>) 
{
  let mut is_line: bool = true;
  let mut is_first: bool = true;
  
  for y in 0..13
  {
    for x in 0..15
    {
      if board[y][x] == CellState::HasWall 
      {
        if y == 0 || y % 2 == 0 
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
              print!("---");
            } 
          } else 
          {
            print!("+");
          }
          continue;
        } else if x == 0 || x % 2 == 0 
        {
          print!("|");
        }
      } else if board[y][x] == CellState::Empty 
      {
        print!("   ");
      } else if board[y][x] == CellState::Occupied(Player::Player1) 
      {
        print!("{}", "▇▇▇".bright_yellow());
      }  else if board[y][x] == CellState::Occupied(Player::Player2) 
      {
        print!("{}", "▇▇▇".bright_blue());
      }
    }

    
    is_first = true;
    println!();
  }
}

fn set_up_board(board: &mut Vec<Vec<CellState>>) 
{
  for y in 0..13 
  {
    for x in 0..15 
    {
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      //  wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall  wall wall
      //  wall empty wall empty wall empty wall empty wall empty wall empty wall empty wall
      // 
      if y == 0 || y % 2 == 0 || x == 0 || x % 2 == 0 
      {
        board[y][x] = CellState::HasWall;
        continue
      }
    }
  }
}

fn main() 
{
  let mut board: Vec<Vec<CellState>> = vec![vec![CellState::Empty; 15]; 13];
  set_up_board(&mut board);
  println!("{:#?}", board);
  print_board(&board);
}
