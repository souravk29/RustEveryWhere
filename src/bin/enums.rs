
#[derive(Debug)]
enum CurrentType {
    AlternatingCurrent(i32),
    DirectCurrent(i32),
}

#[derive(Debug)]
enum ParticleNature{
    Linear,
    Wave,
}

#[derive(Debug)]
struct DefForWaveNature{
    nature_type_at_mps: ParticleNature,
    direction: String,
}

fn main(){

    let nat1 = DefForWaveNature{
        nature_type_at_mps: ParticleNature::Linear,
        direction: String::from("vertical")
    };

    let nat2 = DefForWaveNature{
        nature_type_at_mps: ParticleNature::Wave,
        direction: String::from("horizontal"),
    };

    println!("{:?}", nat1);
    println!("{:?}", nat2);

    let device1 = CurrentType::AlternatingCurrent(10);
    let device2 = CurrentType::DirectCurrent(100);

    println!("{:?}", device1);
    println!("{:?}", device2);

    let c1 = ParticleNature::Linear;
    let c2 = ParticleNature::Wave;

    println!("{:?}", c1);
    println!("{:?}", c2);



}