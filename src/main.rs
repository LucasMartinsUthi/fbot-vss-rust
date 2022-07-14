use fbot_vss_rust::fbot_fira::{fira_protos, get_ball, get_yellow_robot, Robot, Team};
use flo_curves::{bezier::Curve, Coord2, BezierCurve};

fn main() {
    loop {
        let goalie = Robot::new(0, Team::Yellow);
        let ball = get_ball();

        let curve = Curve {
            start_point: Coord2 (goalie.get_x(), goalie.get_y()),
            end_point: Coord2 (ball.x, ball.y),
            control_points: (Coord2 (0.0, 0.0), Coord2 (0.0, 0.0) )
        };


        println!("{:?}", curve.point_at_pos(0.1));

    }
}
