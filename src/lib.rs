use fbot_rust_client::fira_protos;
use fbot_rust_client::{ball, yellow_robot, blue_robot};

const ORIENTATION_KP: f64 = 10.0;
const ROBOT_SPEED: f64 = 20.0;

#[derive(Debug)]
pub enum Team{
    Yellow,
    Blue
}

// TODO
// Implement cord funcitons inside point
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn orientation_to(&self, p: &Point) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        y.atan2(x)
    }

    pub fn distance_to(&self, p: &Point) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        (x*x + y*y).sqrt()
    }
}

#[derive(Debug)]
pub struct Robot {
    id: u32,
    team: Team,
}

impl Robot {
    pub fn new(id: u32, team: Team) -> Self {
        Self {
            id: id,
            team: team,
        }
        }

    fn robot(&self) -> fira_protos::Robot{
        match self.team {
            Team::Yellow => yellow_robot(&self.id).unwrap(),
            Team::Blue => blue_robot(&self.id).unwrap()
        }
    }

    pub fn x(&self) -> f64 {
        self.robot().x
    }

    pub fn y(&self) -> f64 {
        self.robot().y
    }

    pub fn orientation(&self) -> f64 {
        self.robot().orientation
    }

    pub fn control_point(&self) -> (f64, f64) {
        let CP = 0.1;
        let (x, y, orientation) = (self.x(), self.y(), self.orientation());

        let cp_x = orientation.cos() * CP;
        let cp_y = orientation.sin() * CP;

        (x + cp_x, y + cp_y)
    }

    pub fn set_speed(&self, wheel_left: f64, wheel_right: f64) {
        let commands = fira_protos::Commands {
            robot_commands: vec![
                fira_protos::Command {
                    id: 0,
                    yellowteam: true,
                    wheel_left: wheel_left,
                    wheel_right: wheel_right,
                },
            ]
        };

        fbot_rust_client::send_command(commands);
    }

    pub fn go_to(&self, target_x: f64, target_y:f64) {
        loop {
            let diff_x = target_x - self.x();
            let diff_y = target_y - self.y();

            let dist = (diff_x*diff_x + diff_y*diff_y).sqrt();

            if dist < 0.08 {
                break;
            }

            let mut goalie_orientation = self.orientation();
            let target_orientation = (diff_y/diff_x).atan();

            // ele tem problema ao calcular a diferença entre os quadrantes 1 2 e 3 4, ele não rataciona pelo menor caminho
            
            if diff_x < 0.0 {
                if goalie_orientation > 0.0 {
                    goalie_orientation -= std::f64::consts::PI;
                } else {
                    goalie_orientation += std::f64::consts::PI;
                }
            }

            let err = target_orientation - goalie_orientation;
            let velocidade = err * ORIENTATION_KP;
            
            self.set_speed(-velocidade + ROBOT_SPEED, velocidade + ROBOT_SPEED);
        };

        // self.set_speed(0.0, 0.0);

    }
}

// TODO 
// create struct Ball
pub struct Ball {} 

impl Ball {
    pub fn new() -> Self {
        Self {}
    }

    pub fn x(&self) -> f64 {
        ball().x
    }

    pub fn y(&self) -> f64 {
        ball().y
    }

    pub fn control_point(&self) -> (f64, f64){
        let ball = ball();
        let ball_point = Point::new(ball.x, ball.y);
    
        let GOAL_POINT = Point::new(-0.75, 0.0);
        let CP = 0.5;
        
        let orientation_to_goal = ball_point.orientation_to(&GOAL_POINT);
        
        let cp_x = orientation_to_goal.cos() * CP;
        let cp_y = orientation_to_goal.sin() * CP;
    
        (ball.x + cp_x, ball.y + cp_y)
    }
}

