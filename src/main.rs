struct Polyhedra { A:Vec<Vec<i64>>, b:Vec<i64>, c:Option<Vec<i64>> }
impl Polyhedra {
    fn variables(&self) -> usize {
        self.A[0].len()
    }

    fn constraints(&self) -> usize {
        self.A.len()
    }
}

fn main() {
//    println!("Hello, world!");
//    let a = vec! [
//        vec! [1,2,3],
//        vec! [1,2,0],
//        vec! [4,-3,-8],
//    ];
//    let b = vec! [5,4,3];
//    let c = None;
//    let new_a = fme(generate_exercise_2(3)).A;
//    for i in new_a {
//        println!("{:?}", i);
//    }

    let p = 3;

    let mut x = generate_exercise_2(p);

//    for i in 0..p
//    {
//        x = fme(x);
//    }
    x=fme(x);

    for rows in x.A
    {
        println!("{:?}", rows);
    }
}

fn generateAllPossibleSigns() -> Vec<(i64, i64, i64)> {
    let mut ret :Vec<(i64, i64, i64)> = Vec::new();
    let values = vec! [-1,1];
    for i in values.iter()
    {
        for j in values.iter()
        {
            for k in values.iter()
            {
                ret.push((*i,*j,*k));
            }
        }
    }
    ret
}

fn generate_exercise_2(p: usize) -> Polyhedra {
    let n = 2_usize.pow(p as u32) + p + 2;
    let mut a :Vec<Vec<i64>> = Vec::new();
    let mut b : Vec<i64> = Vec::new();

    for k in 0..n
    {
        for j in 0..k
        {
            for i in 0..j
            {
                // 0<=i<k<j<=n
                let mut row = vec! [0; n];
                for l in 0..8 {
                    row[i]= if l & 1 == 0 { 1 } else { -1 };
                    row[j]= if l & 2 == 0 { 1 } else { -1 };
                    row[k]= if l & 4 == 0 { 1 } else { -1 };
                    a.push(row.clone());
                    b.push(1);
                }
            }
        }
    }
    //println!("A has {} rows and {} columns",a.len(), a[0].len());
    Polyhedra{A:a,b,c:None}
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
    assert_eq!(positive.len()+zero.len()+negative.len(), constraints);
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
            let variables = p.variables();
            for var in 0..variables-1 {
                //println!("access p.a[{}][{}] and p.a[{}][{}]", var, *k, var, i);
                new_constraint.push(p.A[*k][var] + p.A[i][var]);
            }
            new_a.push(new_constraint);
            new_b.push(p.b[i]+p.b[*k]);
        }
    }

    for z in zero
    {
        //the rows z doesn't change
        let mut row = p.A[z].clone();
        row.pop();
        new_a.push(row);
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
