use good_lp::{Expression, ProblemVariables, Solution as Sol, SolverModel, Variable, variable};
use num_traits::{One, PrimInt, Signed, Zero};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GF2(u8);

impl GF2 {
    fn new(value: u8) -> Self {
        GF2(value)
    }

    fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for GF2 {
    fn from(value: u8) -> Self {
        GF2::new(value & 1)
    }
}

impl std::ops::Add for GF2 {
    type Output = GF2;

    fn add(self, other: GF2) -> GF2 {
        GF2::new(self.0 ^ other.0)
    }
}

impl std::ops::Sub for GF2 {
    type Output = GF2;

    fn sub(self, other: GF2) -> GF2 {
        GF2::new(self.0 ^ other.0)
    }
}

impl std::ops::Mul for GF2 {
    type Output = GF2;

    fn mul(self, other: GF2) -> GF2 {
        GF2::new(self.0 & other.0)
    }
}

impl std::ops::Div for GF2 {
    type Output = GF2;

    fn div(self, rhs: Self) -> Self::Output {
        GF2::new(self.0 & rhs.0)
    }
}

impl Zero for GF2 {
    fn zero() -> Self {
        GF2::new(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl One for GF2 {
    fn one() -> Self {
        GF2::new(1)
    }
}

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Resolution error: {0}")]
    ResolutionError(#[from] good_lp::ResolutionError),
    #[error("Invalid contraint: {0}")]
    InvalidConstraint(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

#[derive(Debug, Clone)]
struct Model<T> {
    vars: Vec<T>,
    eqs: Vec<Expr<T>>,
}

impl<T: Signed + PrimInt + std::fmt::Display + Into<f64>> Model<T> {
    fn new(num_var: usize) -> Self {
        Model {
            vars: vec![T::zero(); num_var],
            eqs: vec![],
        }
    }

    fn constraint(&mut self, expr: Expr<T>) -> Result<(), ModelError> {
        match expr {
            Expr::BinExpr(bin_expr) => {
                if bin_expr.op != BinOp::Equal {
                    return Err(ModelError::NotImplemented(format!("Only equality constraints are supported, got {:?}", bin_expr.op)));
                }

                self.eqs.push(Expr::BinExpr(bin_expr));
            },
            _ => return Err(ModelError::InvalidConstraint(format!("Expected binary expression, got {}", expr))),
        }

        Ok(())
    }

    fn solve_relaxed(&self, bounds: &Vec<(i32, Option<i32>)>) -> Result<Solution, ModelError> {
        let mut vars = ProblemVariables::new();
        let xs: Vec<Variable> = (0..self.vars.len())
            .map(|i| {
                let (lower, upper) = bounds[i];
                let var = variable().min(lower as f64);
                let var = match upper {
                    Some(upper) => var.max(upper as f64),
                    None => var,
                };
                vars.add(var)
            })
            .collect();

        let objective: Expression = xs.iter().copied().sum();
        
        let mut problem = vars.minimise(objective.clone())
            .using(good_lp::highs);

        for eq in &self.eqs {
            let eq = standardize(eq.clone());
            problem = add_constraint(problem, &eq, &xs)?;
        }

        let solution = problem.solve()?;
        let values = xs.iter().map(|&x| solution.value(x)).collect::<Vec<f64>>();
        let obj = objective.eval_with(&solution);

        let solution = Solution::new(values, obj);
        Ok(solution)
    }
}

fn to_lp_expr<T: Signed + PrimInt + Into<f64>>(
    expr: &Expr<T>,
    xs: &[Variable],
) -> Expression {
    match expr {
        Expr::Var(v) => xs[v.id].into(),
        Expr::Const(c) => Expression::from(c.value.into()),
        Expr::ScalExpr(se) => {
            let a: f64 = se.a.into();
            a * to_lp_expr(&se.expr, xs)
        }
        Expr::SumExpr(sum) => {
            sum.exprs
                .iter()
                .map(|e| to_lp_expr(e, xs))
                .sum()
        }
        Expr::BinExpr(_) => {
            panic!("BinExpr must be handled at constraint level")
        }
    }
}

fn add_constraint<T: Signed + PrimInt + Into<f64>, S: SolverModel>(
    pb: S,
    expr: &Expr<T>,
    xs: &[Variable],
) -> Result<S, ModelError> {
    match expr {
        Expr::BinExpr(bin) => {
            let lhs = to_lp_expr(&bin.left, xs);
            let rhs = to_lp_expr(&bin.right, xs);

            Ok(match bin.op {
                BinOp::Equal => pb.with(lhs.eq(rhs)),
                BinOp::LessThanOrEqual => pb.with(lhs.leq(rhs)),
                BinOp::GreaterThanOrEqual => pb.with(lhs.geq(rhs)),
            })
        }
        _ => Err(ModelError::InvalidConstraint(
            "Expected BinExpr".into(),
        )),
    }
}


#[derive(Debug, Clone)]
enum Expr<T> {
    Var(Var),
    Const(Const<T>),
    ScalExpr(ScalExpr<T>),
    BinExpr(BinExpr<T>),
    SumExpr(SumExpr<T>),
}

impl<T: Signed + PrimInt> Expr<T> {
    fn var(id: usize) -> Self {
        Expr::Var(Var::new(id))
    }

    fn constant(value: T) -> Self {
        Expr::Const(Const::new(value))
    }

    fn sum(exprs: Vec<Expr<T>>) -> Self {
        Expr::SumExpr(SumExpr::new(exprs))
    }

    fn equal(left: Expr<T>, right: Expr<T>) -> Self {
        Expr::BinExpr(BinExpr::eq(left, right))
    }

    fn scale(scalar: T, expr: Expr<T>) -> Self {
        Expr::ScalExpr(ScalExpr::new(scalar, expr))
    }

    fn eq(self, other: Expr<T>) -> Self {
        Expr::equal(self, other)
    }
}

impl<T: Signed + PrimInt + std::fmt::Display> std::fmt::Display for Expr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Const(c) => write!(f, "{}", c),
            Expr::ScalExpr(se) => write!(f, "{}", se),
            Expr::BinExpr(be) => write!(f, "{}", be),
            Expr::SumExpr(sume) => write!(f, "{}", sume),
        }
    }
}

impl<T: Signed + PrimInt> std::ops::Add for Expr<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Expr::BinExpr(_) => {
                panic!("Cannot add to a binary expression")
            },
            Expr::SumExpr(sum_expr) => {
                match rhs {
                    Expr::SumExpr(rhs_sum_expr) => {
                        let mut new_exprs = sum_expr.exprs;
                        for expr in rhs_sum_expr.exprs {
                            new_exprs.push(expr);
                        }
                        Expr::SumExpr(SumExpr::new(new_exprs))
                    },
                    _ => {
                        let mut new_exprs = sum_expr.exprs;
                        new_exprs.push(rhs);
                        Expr::SumExpr(SumExpr::new(new_exprs))
                    }
                }
            },
            _ => Expr::sum(vec![self, rhs]),
        }
    }
}

impl<T: Signed + PrimInt> std::ops::Sub for Expr<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Expr::BinExpr(_) => panic!("Cannot subtract from a binary expression"),
            Expr::SumExpr(sum_expr) => {
                let mut new_exprs = sum_expr.exprs;
                new_exprs.push(Expr::constant(-T::one()) * rhs);
                Expr::SumExpr(SumExpr::new(new_exprs))
            },
            _ => Expr::sum(vec![self, Expr::constant(-T::one()) * rhs]),
        }
    }
}

impl<T: Signed + PrimInt> std::ops::Mul for Expr<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Expr::Const(con) => return Expr::scale(con.value, rhs),
            _ => {}
        }

        match rhs {
            Expr::Const(con) => Expr::scale(con.value, self),
            _ => panic!("Multiplication is only supported by constants."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Var {
    id: usize,
}

impl Var {
    fn new(id: usize) -> Self {
        Var { id }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x[{}]", self.id)
    }
}

#[derive(Debug, Clone, Copy)]
struct Const<T> {
    value: T,
}

impl<T: Signed + PrimInt + std::fmt::Display> std::fmt::Display for Const<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: Signed + PrimInt> Const<T> {
    fn new(value: T) -> Self {
        Const { value }
    }
}

#[derive(Debug, Clone)]
struct ScalExpr<T> {
    a: T,
    expr: Box<Expr<T>>,
}

impl<T: Signed + PrimInt> ScalExpr<T> {
    fn new(scalar: T, expr: Expr<T>) -> Self {
        let mut a = scalar;
        let expr = match expr {
            Expr::ScalExpr(scal_expr) => {
                a = a * scal_expr.a;
                *scal_expr.expr
            },
            _ => expr,
        };

        ScalExpr {
            a,
            expr: Box::new(expr),
        }
    }
}

impl<T: Signed + PrimInt + std::fmt::Display> std::fmt::Display for ScalExpr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.a == T::one() {
            write!(f, "{}", self.expr)
        } else {
            write!(f, "{}*({})", self.a, self.expr)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
    Equal,
    #[allow(dead_code)]
    LessThanOrEqual,
    #[allow(dead_code)]
    GreaterThanOrEqual,
}


#[derive(Debug, Clone)]
struct BinExpr<T> {
    left: Box<Expr<T>>,
    right: Box<Expr<T>>,
    op: BinOp,
}

impl<T: Signed + PrimInt> BinExpr<T> {
    fn new(left: Expr<T>, right: Expr<T>, op: BinOp) -> Self {
        BinExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        }
    }

    fn eq(left: Expr<T>, right: Expr<T>) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);
        let op = BinOp::Equal;
        BinExpr {
            left,
            right,
            op,
        }
    }
}

impl<T: Signed + PrimInt + std::fmt::Display> std::fmt::Display for BinExpr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) = ({})", self.left, self.right)
    }
}

#[derive(Debug, Clone)]
struct SumExpr<T> {
    exprs: Vec<Expr<T>>,
}

impl<T: Signed + PrimInt> SumExpr<T> {
    fn new(exprs: Vec<Expr<T>>) -> Self {
        let mut new_exprs = Vec::new();
        for expr in exprs {
            match expr {
                Expr::SumExpr(sum_expr) => {
                    for e in sum_expr.exprs {
                        new_exprs.push(e);
                    }
                },
                Expr::Const(con) if con.value.is_zero() => {
                    // skip zero
                },
                _ => new_exprs.push(expr),
            }
        }

        SumExpr {
            exprs: new_exprs,
        }
    }
}

impl<T: Signed + PrimInt + std::fmt::Display> std::fmt::Display for SumExpr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr_strings: Vec<String> = self.exprs.iter().map(|e| format!("{}", e)).collect();
        write!(f, "{}", expr_strings.join(" + "))
    }
}

fn fold<T: Signed + PrimInt>(expr: Expr<T>) -> Expr<T> {
    match expr {
        Expr::ScalExpr(scal_expr) => {
            if scal_expr.a.is_zero() {
                Expr::constant(T::zero())
            }
            else if scal_expr.a == T::one() {
                fold(*scal_expr.expr)
            }
            else {
                match fold(*scal_expr.expr) {
                    Expr::Const(con) => Expr::constant(scal_expr.a * con.value),
                    other_expr => Expr::ScalExpr(ScalExpr::new(scal_expr.a, other_expr))
                }
            }
        },
        Expr::BinExpr(bin_expr) => {
            let left = fold(*bin_expr.left);
            let right = fold(*bin_expr.right);
            Expr::BinExpr(BinExpr::new(left, right, bin_expr.op))
        },
        Expr::SumExpr(sum_expr) => {
            let mut sum = T::zero();
            let mut new_exprs = Vec::new();
            for expr in sum_expr.exprs {
                match fold(expr) {
                    Expr::Const(con) => {
                        sum = sum + con.value;
                    },
                    other_expr => {
                        new_exprs.push(other_expr);
                    }
                }
            }

            if !sum.is_zero() {
                new_exprs.push(Expr::constant(sum));
            }

            if new_exprs.len() == 1 {
                new_exprs.pop().unwrap()
            }
            else {
                Expr::sum(new_exprs)
            }
        },
        _ => expr,
    }
}

fn distribute<T: Signed + PrimInt>(expr: Expr<T>) -> Expr<T> {
    match expr {
        Expr::ScalExpr(scal_expr) => {
            match scal_expr.expr.as_ref() {
                Expr::SumExpr(sum_expr) => {
                    let mut new_exprs = Vec::new();
                    for expr in &sum_expr.exprs {
                        new_exprs.push(Expr::scale(scal_expr.a, expr.clone()));
                    }
                    Expr::SumExpr(SumExpr::new(new_exprs))
                },
                _ => Expr::ScalExpr(scal_expr),
            }
        },
        Expr::BinExpr(bin_expr) => {
            let left = distribute(*bin_expr.left);
            let right = distribute(*bin_expr.right);
            Expr::BinExpr(BinExpr::new(left, right, bin_expr.op))
        },
        Expr::SumExpr(sum_expr) => {
            let mut new_exprs = Vec::new();
            for expr in sum_expr.exprs {
                new_exprs.push(distribute(expr));
            }
            Expr::SumExpr(SumExpr::new(new_exprs))
        },
        _ => expr,
    }
}

fn _move_left<T: Signed + PrimInt>(expr: Expr<T>) -> Expr<T> {
    match expr {
        Expr::BinExpr(bin_expr) => {
            if matches!(bin_expr.op, BinOp::LessThanOrEqual) {
                match *bin_expr.right {
                    Expr::SumExpr(sum_expr) => {
                        let mut moved_exprs = Vec::new();
                        let mut stayed_exprs = Vec::new();
                        for expr in sum_expr.exprs {
                            match expr {
                                Expr::Const(con) if con.value.is_negative() => {
                                    moved_exprs.push(Expr::constant(-con.value));
                                },
                                _ => {
                                    stayed_exprs.push(expr);
                                }
                            }
                        }

                        let mut new_lhs = vec![*bin_expr.left];
                        new_lhs.extend(moved_exprs);
                        let lhs = Expr::sum(new_lhs);
                        let rhs = Expr::sum(stayed_exprs);
                        Expr::BinExpr(BinExpr::new(lhs, rhs, BinOp::LessThanOrEqual))
                    },
                    Expr::ScalExpr(scal_expr) => {
                        match *scal_expr.expr {
                            Expr::Const(con) if con.value.is_negative() => {
                                let lhs = Expr::sum(vec![*bin_expr.left, Expr::constant(-scal_expr.a * con.value)]);
                                let rhs = Expr::constant(T::zero());
                                Expr::BinExpr(BinExpr::new(lhs, rhs, BinOp::LessThanOrEqual))
                            },
                            _ => {
                                Expr::BinExpr(BinExpr::new(*bin_expr.left, Expr::ScalExpr(scal_expr), BinOp::LessThanOrEqual))
                            },
                        }
                    }
                    _ => Expr::BinExpr(bin_expr),
                }
            }
            else{
                Expr::BinExpr(bin_expr)
            }
        },
        _ => expr,
    }
}

fn standardize<T: Signed + PrimInt>(expr: Expr<T>) -> Expr<T> {
    let expr = distribute(expr);
    //let expr = move_left(expr); // Not needed for equality constraints
    let expr = fold(expr);
    expr
}

#[derive(Debug, Clone)]
struct SubProblem {
    model: Model<i32>,          // equations (unchanged)
    bounds: Vec<(i32, Option<i32>)>, // (lower, upper)
}

impl SubProblem {
    fn new(model: Model<i32>, bounds: Vec<(i32, Option<i32>)>) -> Self {
        SubProblem { model, bounds }
    }

    fn solve_lp_relaxed(&self) -> Option<Solution> {
        self.model.solve_relaxed(&self.bounds).ok()
    }
}

#[derive(Debug)]
struct Solution {
    values: Vec<f64>,
    obj: f64,
}

impl Solution {
    fn new(values: Vec<f64>, obj: f64) -> Self {
        Solution { values, obj }
    }
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    lights_goal: Vec<bool>,
    actions: Vec<Vec<usize>>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let mut input = input.chars().into_iter();
        let mut lights_goal = vec![];
        for c in &mut input {
            match c {
                '#' => lights_goal.push(true),
                '.' => lights_goal.push(false),
                ']' => break,
                _ => continue,
            }
        }

        let lights = vec![false; lights_goal.len()];
        let mut actions = vec![];
        let mut current_action = vec![];
        let mut buffer = String::new();
        for c in &mut input {
            match c {
                '0'..='9' => buffer.push(c),
                ',' => {
                    if !buffer.is_empty() {
                        let index = buffer.parse::<usize>().unwrap();
                        current_action.push(index);
                        buffer.clear();
                    }
                }
                ')' => {
                    if !buffer.is_empty() {
                        let index = buffer.parse::<usize>().unwrap();
                        current_action.push(index);
                        buffer.clear();
                    }
                    actions.push(current_action);
                    current_action = vec![];
                }
                '{' => break,
                _ => continue,
            }
        }

        let mut joltage_requirements = vec![];
        for c in &mut input {
            match c {
                '0'..='9' => {
                    buffer.push(c);
                }
                ',' => {
                    if !buffer.is_empty() {
                        let joltage = buffer.parse::<u16>().unwrap();
                        joltage_requirements.push(joltage);
                        buffer.clear();
                    }
                }
                '}' => {
                    if !buffer.is_empty() {
                        let joltage = buffer.parse::<u16>().unwrap();
                        joltage_requirements.push(joltage);
                        buffer.clear();
                    }
                    break;
                }
                _ => continue
            }
        }

        println!("Parsed machine: lights_goal={:?}, actions={:?}, joltage_requirements={:?}", lights_goal, actions, joltage_requirements);

        Machine { 
            lights,
            lights_goal,
            actions,
            joltage_requirements,
        }
    }

    fn solve_part1(&self) -> Option<Vec<GF2>> {
        let rows = self.lights.len();
        let cols = self.actions.len();
        let mut actions = vec![vec![GF2::zero(); cols + 1]; rows];
        // Setup action matrix
        for (j, action) in self.actions.iter().enumerate() {
            for &i in action {
                actions[i][j] = GF2::one();
            }
        }

        // Augmented column for goal state
        for i in 0..rows {
            actions[i][cols] = if self.lights[i] == self.lights_goal[i] {
                GF2::zero()
            }
            else {
                GF2::one()
            };
        }

        self.print_and_solve_part1(&mut actions, false)
    }

    // For debugging
    fn print_and_solve_part1(&self, actions: &mut Vec<Vec<GF2>>, print: bool) -> Option<Vec<GF2>> {
        if print {
            println!("Target machine state: {:?}", self.lights_goal);
            println!("Actions: {:?}", self.actions);
            println!("Before reduction:");
            print_matrix(actions);
            rref_gf2(actions);
            println!("After reduction:");
            print_matrix(actions);
        }
        else{
            rref_gf2(actions);
        }

        let solution = extract_shortest_solution(actions);
        solution
    }

    fn solve_part2(&self) -> Option<Solution> {
        let model = construct_model(&self.actions, &self.joltage_requirements, false);
        let bounds = vec![(0, None); self.actions.len()];
        let subproblem = SubProblem::new(model, bounds);
        //let dummy_solution = Solution::new(vec![], f64::INFINITY);

        let solution = branch_and_bound(subproblem, None);
        debug_assert!(solution.is_some(), "No solution found for part 2");
        debug_assert!(
                solution.as_ref().unwrap().values.iter().all(|&v| is_int(v, 1e-8)),
                "Non-integer solution escaped B&B"
        );

        solution
    }
}

fn branch_and_bound(subproblem: SubProblem, best: Option<Solution>) -> Option<Solution> {
    const TOLERANCE: f64 = 1e-8;

    let solution = match subproblem.solve_lp_relaxed() {
        Some(s) => s,
        None => return best, // infeasible LP
    };

    if let Some(ref b) = best {
        if solution.obj >= b.obj {
            return best; // valid prune
        }
    }

    if solution.values.iter().all(|&v| is_int(v, TOLERANCE)) {
        return match best {
            None => Some(solution),
            Some(b) if solution.obj < b.obj => Some(solution),
            Some(b) => Some(b),
        };
    }

    // cutter logic

    // branch on first fractional variable
    let (ix, val) = solution.values.iter()
        .enumerate()
        .find(|(_, v)| !is_int(**v, TOLERANCE))
        .unwrap();

    let (left, right) = branch(subproblem, ix, *val);

    let best = branch_and_bound(left, best);
    branch_and_bound(right, best)
}

fn branch(
    sub: SubProblem,
    var: usize,
    value: f64,
) -> (SubProblem, SubProblem) {
    let lo = value.floor() as i32;
    let hi = value.ceil() as i32;

    let mut left = sub.clone();
    left.bounds[var].1 = Some(
        left.bounds[var].1.map_or(lo, |ub| ub.min(lo))
    );

    let mut right = sub.clone();
    right.bounds[var].0 = right.bounds[var].0.max(hi);

    (left, right)
}

fn construct_model(actions: &Vec<Vec<usize>>, joltage_requirements: &Vec<u16>, debug: bool) -> Model<i32> {
    if debug {
        println!("Actions: {:?}", actions);
        println!("Joltage requirements: {:?}", joltage_requirements);
    }

    let mut model = Model::<i32>::new(actions.len());
    // Create a vec of vecs where each row corresponds to an action and each column to a joltage requirement index affected
    let mut coeffs = vec![vec![0i32; joltage_requirements.len()]; actions.len()];
    for (i, actions) in actions.iter().enumerate() {
        for action_index in actions {
            coeffs[i][*action_index] = 1;
        }
    }

    if debug {
        println!("Action coefficients matrix:");
        for row in coeffs.iter() {
            println!("{:?}", row);
        }
        println!();
    }

    for (i, &req) in joltage_requirements.iter().enumerate() {
        if debug {
            println!("Joltage requirement {}: {}", i, req);
        }

        let mut expr = vec![];
        for (id, coeff) in coeffs.iter().enumerate() {
            if coeff[i] != 0 {
                expr.push(Expr::scale(coeff[i], Expr::var(id)));
            }
        }

        let lhs = Expr::sum(expr);
        let rhs = Expr::constant(req as i32);
        let constraint = lhs.eq(rhs);
        model.constraint(constraint).unwrap();
    }

    if debug {
        //println!("Model constructed for part 2: {:?}", model);
        println!("Equations constructed:");
        for eq in model.eqs.iter() {
            println!("{}", eq);
        }
    }

    model
}

fn is_int(value: f64, tol: f64) -> bool {
    (value - value.round()).abs() < tol
}

fn print_matrix(mat: &Vec<Vec<GF2>>) {
    let cols = mat[0].len();
    for _ in 0..cols {
        print!("--");
    }
    println!();

    for row in mat.iter() {
        for val in row.iter() {
            print!("{} ", val.get());
        }
        println!();
    }

    for _ in 0..cols {
        print!("--");
    }
    println!();
}

fn rref_gf2(mat: &mut Vec<Vec<GF2>>) -> Vec<Option<usize>> {
    let m = mat.len();
    let n = mat[0].len();

    let mut pivot_col_for_row = vec![None; m];

    // Forward elimination (REF)
    let mut r = 0;
    for c in 0..n {
        if r >= m {
            break;
        }

        // Find pivot row with 1 in column c
        let mut pivot = None;
        for i in r..m {
            if !mat[i][c].is_zero() {
                pivot = Some(i);
                break;
            }
        }
        if pivot.is_none() {
            continue;
        }

        let p = pivot.unwrap();
        mat.swap(r, p);
        pivot_col_for_row[r] = Some(c);

        // Zero out below
        for i in (r + 1)..m {
            if !mat[i][c].is_zero() {
                for j in c..n {
                    mat[i][j] = mat[i][j] + mat[r][j]; // XOR
                }
            }
        }

        r += 1;
    }

    // Backward elimination (make it RREF)
    // For each pivot row from bottom to top, clear above
    for i in (0..m).rev() {
        let Some(pc) = pivot_col_for_row[i] else {
            continue;
        };

        for k in 0..i {
            if !mat[k][pc].is_zero() {
                for j in pc..n {
                    mat[k][j] = mat[k][j] + mat[i][j]; // XOR
                }
            }
        }
    }

    pivot_col_for_row
}

fn extract_shortest_solution(rref: &[Vec<GF2>]) -> Option<Vec<GF2>> {
    let rows = rref.len();
    let cols = rref[0].len() - 1; // exclude augmented b column

    // Step 1: identify pivot columns
    let mut pivot_col_for_row = vec![None; rows];
    let mut is_pivot_col = vec![false; cols];

    for r in 0..rows {
        for c in 0..cols {
            if rref[r][c] == GF2::one() {
                pivot_col_for_row[r] = Some(c);
                is_pivot_col[c] = true;
                break;
            }
        }
    }

    // Step 2: collect free columns
    let free_cols: Vec<usize> = (0..cols)
        .filter(|&c| !is_pivot_col[c])
        .collect();

    let f = free_cols.len();
    let mut best_solution: Option<Vec<GF2>> = None;
    let mut best_weight = usize::MAX;

    // Step 3: enumerate free-variable assignments
    for mask in 0..(1 << f) {
        let mut x = vec![GF2::zero(); cols];

        // assign free vars
        for (i, &col) in free_cols.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                x[col] = GF2::one();
            }
        }

        // compute pivot vars
        for r in 0..rows {
            let Some(pc) = pivot_col_for_row[r] else { continue };

            let mut value = rref[r][cols]; // b

            for c in 0..cols {
                if c != pc && rref[r][c] == GF2::one() {
                    value = value + x[c];
                }
            }

            x[pc] = value;
        }

        // Step 4: count moves (Hamming weight)
        let weight = x.iter().filter(|v| **v == GF2::one()).count();

        if weight < best_weight {
            best_weight = weight;
            best_solution = Some(x);
        }
    }

    best_solution
}

pub fn solve_day10_puzzle_part1() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let machines = input.lines()
        .map(|line| Machine::from_str(line))
        .collect::<Vec<Machine>>();
    
    let mut fewest_moves = 0;
    for machine in machines.iter() {
        let solution = machine.solve_part1();
        //println!("Solution: {:?}", solution);
        if let Some(sol) = solution {
            let moves = sol.iter().filter(|v| **v == GF2::one()).count();
            fewest_moves += moves;
        }
    }

    println!("Total fewest moves for all machines: {}", fewest_moves);

    Ok(())
}

pub fn solve_day10_puzzle_part2() -> Result<(), PuzzleError> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let machines = input.lines()
        .map(|line| Machine::from_str(line))
        .collect::<Vec<Machine>>();
    
    let mut fewest_moves = 0;
    for machine in machines.iter() {
        let solution = machine.solve_part2();
        if let Some(sol) = solution {
            //println!("Solution: {:?}", sol);
            //let moves: u32 = sol.values.iter().map(|&v| v as u32).sum();
            let moves: usize = sol
                .values
                .iter()
                .map(|&v| v.round() as usize)
                .sum();

            println!("Actions: {:?}, Target: {:?}, Moves: {}", machine.actions, machine.joltage_requirements, moves);
            fewest_moves += moves;
        }
    }

    println!("Total fewest moves for all machines: {}", fewest_moves);

    Ok(())
}

#[test]
fn test_day10_part1() {
    assert!(solve_day10_puzzle_part1().is_ok());
}

#[test]
fn test_day10_part2() { 
    assert!(solve_day10_puzzle_part2().is_ok());
}