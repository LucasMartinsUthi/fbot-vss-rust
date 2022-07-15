use fbot_vss_rust::fbot_fira::{fira_protos, get_ball, get_yellow_robot, Robot, Team, get_ball_cp};
use flo_curves::{bezier::Curve, Coord2, BezierCurve};
use std::{thread, time};

fn main() {
    let goalie = Robot::new(0, Team::Yellow);
    let ball = get_ball();

    let cp_goalie = goalie.get_control_point();
    let cp_ball = get_ball_cp();


    let curve = Curve {
        start_point: Coord2 (goalie.get_x(), goalie.get_y()),
        end_point: Coord2 (ball.x, ball.y),
        control_points: (Coord2 (cp_goalie.0, cp_goalie.1), Coord2 (cp_ball.0, cp_ball.1))
    };

    for i in 0..20 { 
        let pos: f64 = i as f64 / 20.0;
        
        let point = curve.point_at_pos(pos);
        let (x, y) = (point.0, point.1);

        goalie.go_to(x, y);
    }

    goalie.set_speed(100.0, 100.0);

    
    thread::sleep(time::Duration::from_millis(300));

    goalie.set_speed(0.0, 0.0);
}
