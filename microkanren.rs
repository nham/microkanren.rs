use std::collections::HashMap;

// A state is a pair of a substitution (an associatin list)
// and a non-negative integer, the fresh-variable counter
//
// "In muKanren, as in most miniKanren languages, we use
// triangular substitutions". I think this just means
// representing a substitution of n variables as the composition
// of n substitutions of 1 variable each?

type Var = uint;
type FuncSym = char;

// a term is either a variable
// or a function symbol + n terms, where n is the arity of the function
enum Term<T> {
    Variable(Var),
    Other(T),
    Compound(Vec<T>),
}

impl<T> Term<T> {
    fn is_var(&self) -> bool {
        match *self {
            Variable(_) => true,
            _ => false,
        }
    }

    fn is_compound(&self) -> bool {
        !self.is_var()
    }

    fn unwrap_var(&self) -> Var {
        match *self {
            Variable(v) => v,
            _ => fail!("Term is not a variable, can't unwrap_var"),
        }
    }
}

/*
type Substitution = Vec<(Var, Term)>;
type State = (Substitution, uint)
*/


struct State {
    // in contrast to Substitution, we don't actually
    // give variable names, we just number the variables 0, 1, 2, ...
    bindings: HashMap<Var, Term>,
    count: uint,
}

impl State {
    // 'walk' in the microKanren paper
    fn get_binding<'a>(&'a self, t: &'a Term) -> &'a Term {
        if !t.is_var() {
            t
        } else {
            match self.bindings.find(&t.unwrap_var()) {
                None => t,
                Some(term) => self.get_binding(term),
            }
        }
    }

    // ext-s in the microKanren paper
    fn add_binding(&mut self, v: Var, t: Term) {
        self.bindings.insert(v, t);
    }

    fn unify(&self, a: &Term, b: &Term) -> Option<State> {
        let a = self.get_binding(a);
        let p = self.get_binding(b);

        if a.is_var() && b.is_var() && a == b {
            Some(self.clone())
        } else if a.is_var() {
            let mut s = self.clone();
            s.add_binding(a.clone(), b.clone());
            Some(s)
        } else if b.is_var() {
            let mut s = self.clone();
            s.add_binding(b.clone(), a.clone());
            Some(s)
        } else if a.is_compound() && b.is_compound() {
            // attempt to unify each
        } else if a == b {
            Some(self.clone())
        } else {
            None
        }
    }
}


// empty state is ({}, 0) where '{}' is the representation
// of the empty substitution

// a goal is basically a function
trait Goal<I: Iterator<State>> {
    // applying a goal to a state yields a "stream"
    // of states
    fn apply(s: State) -> I;
}

// four primitive goal constructors:
//   - unify
//   - call/fresh
//   - disj
//   - conj
//
// conj/disj kind of seem like "goal combinators"
// they each take two goals and return a new goal
// 
// unify takes two arguments, maybe, and produces 
// a goal that suuceeds when the two arguments unify
// (i.e. when they can be shown to coincide after
// some substitution)
//
// call/fresh takes a function Variables -> Goals. 
// I currently don't see how this is a goal constructor. the return type isn't a goal? oh,
// it probably has a variable captured inside somehow
// so it takes a (Variables -> Goals) function and
// applies it to the internal variable.


fn main() {

}
