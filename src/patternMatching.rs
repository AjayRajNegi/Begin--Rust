enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn calculate_area(shape: Shape)->f64{
    let ans: f64 = match shape{
        Shape::Circle(radius)=> 3.14 * radius * radius,
        Shape::Rectangle(width, height)=>{
            width*height
        },
        Shape::Square(side)=>side*side
    };

    return ans;
}

fn main(){
    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(3.0, 6.0);

    let area = calculate_area(circle);

print!("Area of circle: {}\n",area);
}