// use fbot_vss_rust::fira_rust::{fira_protos, get_ball, get_yellow_robot, send_command};
use fbot_vss_rust::{Robot, Team, Ball};
use fbot_rust_client::{fira_protos, ball, yellow_robot, send_command};

const ORIENTATION_KP: f64 = 10.0;
const ROBOT_SPEED: f64 = 20.0;

fn main() {
    let goalie = Robot::new(0, Team::Yellow);
    let ball = Ball::new();

    let diff = goalie.point().orientation_to(&ball.point());
        
    println!("Diff: {}", diff);
    
}