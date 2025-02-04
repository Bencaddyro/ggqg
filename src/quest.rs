use std::fmt::{Display,Formatter,Result};
use rand::Rng;


enum ActionTarget {
    Somebody(Somebody),
    Something(Something),
}
impl Default for ActionTarget {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random: bool = rng.gen();
        if random {
            ActionTarget::Somebody(Somebody::default())
        } else {
            ActionTarget::Something(Something::default())
        }
    }
}
impl Display for ActionTarget {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ActionTarget::Somebody(s) => write!(f, "{s}"),
            ActionTarget::Something(s) => write!(f, "the {s}"),
        }
    }
}

struct Somebody {
    name: String,
}
impl Default for Somebody {
    fn default() -> Self { Self {name: String::from("<Name>") } }
}
impl Display for Somebody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}",self.name)
    }
}

struct Something {
    name: String,
}
impl Default for Something {
    fn default() -> Self { Self {name: String::from("<Thing>") } }
}
impl Display for Something {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}",self.name)
    }
}

struct Somewhere {
    name: String,
}
impl Default for Somewhere {
    fn default() -> Self { Self {name: String::from("<Location>") } }
}
impl Display for Somewhere {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}",self.name)
    }
}

// Atomic Action
#[derive(Default)]
struct Capture {
    somebody: Somebody,
}
impl Display for Capture {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "capture {}",self.somebody)
    }
}

#[derive(Default)]
struct Damage {
    some: ActionTarget,
}
impl Display for Damage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "damage {}",self.some)
    }
}

#[derive(Default)]
struct Defend {
    some: ActionTarget,
}
impl Display for Defend {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "defend {}",self.some)
    }
}

#[derive(Default)]
struct Escort {
    somebody: Somebody,
}
impl Display for Escort {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "escort {}",self.somebody)
    }
}

#[derive(Default)]
struct Exchange {
    somebody: Somebody,
    something: Something,
}
impl Display for Exchange {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "exchange for {} with {}",self.something, self.somebody)
    }
}

#[derive(Default)]
struct Experiment {
    something: Something,
}
impl Display for Experiment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "experiment with {}",self.something)
    }
}

#[derive(Default)]
struct Explore {}
impl Display for Explore {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "explore a bit around")
    }
}

#[derive(Default)]
struct Gather {
    something: Something,
}
impl Display for Gather {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "gather for {}",self.something)
    }
}

#[derive(Default)]
struct Give {
    somebody: Somebody,
    something: Something,
}
impl Display for Give {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "give {} to {}",self.something, self.somebody)
    }
}

#[derive(Default)]
struct Goto {
    some: Somewhere,
}
impl Display for Goto {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "go to {} ",self.some)
    }
}

#[derive(Default)]
struct Kill {
    somebody: Somebody,
}
impl Display for Kill {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "kill {}", self.somebody)
    }
}

#[derive(Default)]
struct Listen {
    somebody: Somebody,
}
impl Display for Listen {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "listen {}", self.somebody)
    }
}

#[derive(Default)]
struct r#Read {
    something: Something,
}
impl Display for r#Read {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "read {}",self.something)
    }
}

#[derive(Default)]
struct Repair {
    something: Something,
}
impl Display for Repair {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "repair {}",self.something)
    }
}

#[derive(Default)]
struct Report {
    somebody: Somebody,
}
impl Display for Report {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "report to {}", self.somebody)
    }
}

#[derive(Default)]
struct Spy {
    some: ActionTarget,
}
impl Display for Spy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "spy about {}",self.some)
    }
}

#[derive(Default)]
struct Stealth {
    somebody: Somebody,
}
impl Display for Stealth {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "sneak behind {}",self.somebody)
    }
}

#[derive(Default)]
struct Take {
    somebody: Somebody,
    something: Something,
}
impl Display for Take {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "take the {} from {}",self.something, self.somebody)
    }
}

#[derive(Default)]
struct Use {
    something: Something,
}
impl Display for Use {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "use the {}",self.something)
    }
}

// Rule Action
enum RuleSubQuest {
    Goto(Box<RuleGoto>),
    GotoReturn(Box<RuleGoto>, Box<Quest>, Goto),
}
impl Default for RuleSubQuest {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => RuleSubQuest::Goto(Box::new(RuleGoto::default())),
            _ => RuleSubQuest::GotoReturn(Box::new(RuleGoto::default()), Box::new(Quest::default()), Goto::default())
        }
    }
}
impl Display for RuleSubQuest {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSubQuest::Goto(a) => write!(f, "{a}"),
            RuleSubQuest::GotoReturn(a,b,c) => write!(f, "first {a}, then do {b}, after {c}"),
        }
    }
}

enum RuleGoto {
    None,
    Explore(Explore),
    Learn(Box<RuleLearn>, Goto),
}
impl Default for RuleGoto {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..3);
        match random {
            0 => RuleGoto::None,
            1 => RuleGoto::Explore(Explore::default()),
            _ => RuleGoto::Learn(Box::new(RuleLearn::default()), Goto::default()),
        }
    }
}
impl Display for RuleGoto {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleGoto::None => write!(f, "nothing (You are already at the right place)"),
            RuleGoto::Explore(a) => write!(f, "go {a}"),
            RuleGoto::Learn(a,b) => write!(f, "first learn {a}, then {b}"),
        }
    }
}

enum RuleLearn {
    None,
    SubQuest(Box<RuleGoto>, Box<RuleSubQuest>, Listen),
    r#Read(Box<RuleGoto>, Box<RuleGet>, r#Read),
    Fetch(Box<RuleGet>, Box<RuleSubQuest>, Give, Listen),
}
impl Default for RuleLearn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => RuleLearn::None,
            1 => RuleLearn::SubQuest(Box::new(RuleGoto::default()),Box::new(RuleSubQuest::default()),Listen::default()),
            2 => RuleLearn::r#Read(Box::new(RuleGoto::default()),Box::new(RuleGet::default()),r#Read::default()),
            _ => RuleLearn::Fetch(Box::new(RuleGet::default()),Box::new(RuleSubQuest::default()),Give::default(),Listen::default()),
        }
    }
}
impl Display for RuleLearn {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleLearn::None => write!(f, "nothing (You already know it)"),
            RuleLearn::SubQuest(a,b,c) => write!(f, "first {a}, then do {b}, then {c}"),
            RuleLearn::r#Read(a,b,c) => write!(f, "first {a}, then {b} and {c}"),
            RuleLearn::Fetch(a,b ,c ,d ) => write!(f, "first {a}, then do {b}, {c} and {d}"),
        }
    }
}

enum RuleGet {
    None,
    Steal(Box<RuleSteal>),
    Gather(Box<RuleGoto>, Gather),
    Exchange(
        Box<RuleGoto>,
        Box<RuleGet>,
        Box<RuleGoto>,
        Box<RuleSubQuest>,
        Exchange,
    ),
}
impl Default for RuleGet {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => RuleGet::None,
            1 => RuleGet::Steal(Box::new(RuleSteal::default())),
            2 => RuleGet::Gather(Box::new(RuleGoto::default()),Gather::default()),
            _ => RuleGet::Exchange(Box::new(RuleGoto::default()),Box::new(RuleGet::default()),Box::new(RuleGoto::default()),Box::new(RuleSubQuest::default()),Exchange::default()),
        }
    }
}
impl Display for RuleGet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleGet::None => write!(f, "nothing (You already have it)"),
            RuleGet::Steal(a) => write!(f, "{a}"),
            RuleGet::Gather(a,b) => write!(f, "first {a}, then {b}"),
            RuleGet::Exchange(a,b ,c ,d,e ) => write!(f, "first {a}, then {b}, {c} {d} and {e}"),
        }
    }
}

enum RuleSteal {
    Stealth(Box<RuleGoto>, Stealth, Take),
    Kill(Box<RuleGoto>, Box<RuleKill>, Take),
}
impl Default for RuleSteal {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => RuleSteal::Stealth(Box::new(RuleGoto::default()),Stealth::default(), Take::default()),
            _ => RuleSteal::Kill(Box::new(RuleGoto::default()),Box::new(RuleKill::default()),Take::default()),
        }
    }
}
impl Display for RuleSteal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSteal::Stealth(a, b, c) => write!(f, "{a} {b},then {c}"),
            RuleSteal::Kill(a,b,c) => write!(f, "{a}, then {b} and {c}"),
        }
    }
}

enum  RuleSpy {
    Spy(Box<RuleGoto>, Spy, Box<RuleGoto>, Report)
}
impl Default for RuleSpy {
    fn default() -> Self {
        RuleSpy::Spy(Box::new(RuleGoto::default()),Spy::default(),Box::new(RuleGoto::default()),Report::default())
    }
}
impl Display for RuleSpy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSpy::Spy(a, b, c,d) => write!(f, "{a} {b}, then {c} and {d}"),
        }
    }
}

enum RuleCapture {
    Capture(Box<RuleGet>, Box<RuleGoto>, Capture)
}
impl Default for RuleCapture {
    fn default() -> Self {
        RuleCapture::Capture(Box::new(RuleGet::default()),Box::new(RuleGoto::default()),Capture::default())
    }
}
impl Display for RuleCapture {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleCapture::Capture(a, b, c) => write!(f, "{a} then {b} and {c}"),
        }
    }
}

enum RuleKill {
    Kill(Box<RuleGoto>, Kill)
}
impl Default for RuleKill {
    fn default() -> Self {
        RuleKill::Kill(Box::new(RuleGoto::default()),Kill::default())
    }
}
impl Display for RuleKill {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleKill::Kill(a, b) => write!(f, "{a} then {b} "),
        }
    }
}

// Main Quest
enum Quest {
    Knowledge(Knowledge),
    Comfort(Comfort),
    Reputation(Reputation),
    Serenity(Serenity),
    Protection(Protection),
    Conquest(Conquest),
    Wealth(Wealth),
    Ability(Ability),
    Equipement(Equipement),
}

enum Knowledge {
    Deliver(Box<RuleGet>, Box<RuleGoto>, Give),
    Spy(Box<RuleSpy>),
    Interview(Box<RuleGoto>, Listen, Box<RuleGoto>, Report),
    Use(Box<RuleGet>, Box<RuleGoto>, Use, Box<RuleGoto>, Give),
}
enum Comfort {
    Luxuries(Box<RuleGet>, Box<RuleGoto>, Give),
    Kill(Box<RuleGoto>, Damage, Box<RuleGoto>, Report),
}
enum Reputation {
    Obtain(Box<RuleGet>, Box<RuleGoto>, Give),
    Kill(Box<RuleGoto>, Box<RuleKill>, Box<RuleGoto>, Report),
    Visit(Box<RuleGoto>, Box<RuleGoto>, Report),
}
enum Serenity {
    Revenge(Box<RuleGoto>, Damage),
    Capture1(Box<RuleGet>, Box<RuleGoto>, Use, Box<RuleGoto>, Give),
    Capture2(
        Box<RuleGet>,
        Box<RuleGoto>,
        Use,
        Capture,
        Box<RuleGoto>,
        Give,
    ),
    Check1(Box<RuleGoto>, Listen, Box<RuleGoto>, Report),
    Check2(Box<RuleGoto>, Take, Box<RuleGoto>, Give),
    Recover(Box<RuleGet>, Box<RuleGoto>, Give),
    Rescue(Box<RuleGoto>, Damage, Escort, Box<RuleGoto>, Report),
}
enum Protection {
    Attack(Box<RuleGoto>, Damage, Box<RuleGoto>, Report),
    Treat1(Box<RuleGet>, Box<RuleGoto>, Use),
    Treat2(Box<RuleGoto>, Repair),
    Diversion1(Box<RuleGet>, Box<RuleGoto>, Use),
    Diversion2(Box<RuleGoto>, Damage),
    Assemble(Box<RuleGoto>, Repair),
    Guard(Box<RuleGoto>, Defend),
}
enum Conquest {
    Attack(Box<RuleGoto>, Damage),
    Steal(Box<RuleGoto>, Box<RuleSteal>, Box<RuleGoto>, Give),
}
enum Wealth {
    Gather(Box<RuleGoto>, Box<RuleGet>),
    Steal(Box<RuleGoto>, Box<RuleSteal>),
    Repair(Repair),
}
enum Ability {
    Assemble(Repair, Use),
    Obtain(Box<RuleGet>, Use),
    Use(Use),
    Practice1(Damage),
    Practice2(Use),
    Research1(Box<RuleGet>, Use),
    Research2(Box<RuleGet>, Experiment),
}
enum Equipement {
    Assemble(Repair),
    Deliver(Box<RuleGet>, Box<RuleGoto>, Give),
    Steal(Box<RuleSteal>),
    Trade(Box<RuleGoto>, Exchange),
}
