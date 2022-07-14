use fbot_vss_rust::fbot_fira::{fira_protos, get_ball, get_yellow_robot, Robot, Teams};



fn main() {
    loop {
        let goalie = Robot::new(0, Teams::Yellow);
        println!("{:?}", goalie);
        // let ball = get_ball();

        println!("{:?}", goalie.get_x());
        // goalie.go_to(ball.x, ball.y)
    }
}
