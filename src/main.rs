struct Polyhedra { A:Vec<Vec<i64>>, b:Vec<i64>, c:Option<Vec<i64>> }
impl Polyhedra {
    fn variables(&self) -> usize {
        self.A.len()
    }

    fn constraints(&self) -> usize {
        self.A[0].len()
    }
}

fn main() {
    println!("Hello, world!");
    let a = vec! [
        vec! [1,2,3],
        vec! [1,2,0],
        vec! [4,-3,-8],
    ];
    let b = vec! [5,4,3];
    let c = None;
    let new_a = fme(Polyhedra{A:a,b,c}).A;
    println!("{:?}", new_a);
}

fn partition(p:&Polyhedra) -> (Vec<usize>,Vec<usize>,Vec<usize>){
    let mut positive :Vec<usize> = Vec::new();
    let mut negative :Vec<usize> = Vec::new();
    let mut zero     :Vec<usize> = Vec::new();

    let constraints = p.constraints();

    for (i, x) in p.A.iter().enumerate() {
        let x_n_constraint = x[x.len()-1];
        if x_n_constraint == 0
        {
            zero.push(i);
        }
        else if x_n_constraint > 0
        {
            positive.push(i);
        }
        else
        {
            negative.push(i);
        }
    }
    (positive,zero,negative)
}
fn gen_polyhedra_from_new_constraints(p :Polyhedra, pos:Vec<usize>, zero:Vec<usize>, neg:Vec<usize>) -> Polyhedra {
    let mut new_a : Vec<Vec<i64>> = Vec::new();
    let mut new_b : Vec<i64> = Vec::new();

    for i in pos
    {
        //borrowing from neg
        for k in neg.iter()
        {
            let mut new_constraint :Vec<i64> = Vec::new();
            for constraints in 0..p.constraints() {
                new_constraint.push(p.A[constraints][*k] + p.A[constraints][i]);
            }
            new_a.push(new_constraint);
            new_b.push(p.b[i]+p.b[*k]);
        }
    }

    for z in zero
    {
        //the rows z doesn't change
        new_a.push(p.A[z].clone());
        new_b.push(p.b[z].clone());
    }

    Polyhedra{A:new_a, b:new_b, c:None}
}
// gets a polyhedra with n variables and returns a equivalent polyhedra with n-1 variables
// also called Fourier-Motz-Elimination
fn fme(p:Polyhedra) -> Polyhedra {
    let (positive, zero, negative) = partition(&p);
    return gen_polyhedra_from_new_constraints(p, positive, zero, negative);
}
