fn main() { 
    let v = vec/ [20,40,60,80] 
    // vector v owns the ohject in heap

    let v2 = v;
    let v2 return = display(v2);
    println!("In main {:?}", v);
}

fn display(v:Vec<132>)->Vec<132> { 
    // relurning same vector
    println!("inside display {:?}",v);
    return v;

}