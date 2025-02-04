enum ActionTarget {
    Somebody(Somebody),
    Something(Something),
}

struct Somebody {
    name: String,
}
struct Something {
    name: String,
}
struct Somewhere {
    name: String,
}

// Atomic Action
struct Capture {
    somebody: Somebody,
}
struct Damage {
    some: ActionTarget,
}
struct Defend {
    some: ActionTarget,
}
struct Escort {
    somebody: Somebody,
}
struct Exchange {
    somebody: Somebody,
    something: Something,
}
struct Experiment {
    something: Something,
}
struct Explore {}
struct Gather {
    something: Something,
}
struct Give {
    somebody: Somebody,
    something: Something,
}
struct Goto {
    some: Somewhere,
}
struct Kill {
    somebody: Somebody,
}
struct Listen {
    somebody: Somebody,
}
struct r#Read {
    something: Something,
}
struct Repair {
    something: Something,
}
struct Report {
    somebody: Somebody,
}
struct Spy {
    some: ActionTarget,
}
struct Stealth {
    somebody: Somebody,
}
struct Take {
    somebody: Somebody,
    something: Something,
}
struct Use {
    something: Something,
}

// Rule Action
enum RuleSubQuest {
    Goto(Box<RuleGoto>),
    GotoReturn(Box<RuleGoto>, Box<Quest>, Goto),
}
enum RuleGoto {
    None,
    Explore(Explore),
    Learn(Box<RuleLearn>, Goto),
}
enum RuleLearn {
    None,
    SubQuest(Box<RuleGoto>, Box<RuleSubQuest>, Listen),
    r#Read(Box<RuleGoto>, Box<RuleGet>, r#Read),
    Fetch(Box<RuleGet>, Box<RuleSubQuest>, Give, Listen),
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
enum RuleSteal {
    Stealth(Box<RuleGoto>, Stealth, Take),
    Kill(Box<RuleGoto>, Box<RuleKill>, Take),
}
type RuleSpy = (Box<RuleGoto>, Spy, Box<RuleGoto>, Report);
type RuleCapture = (Box<RuleGet>, Box<RuleGoto>, Capture);
type RuleKill = (Box<RuleGoto>, Kill);

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
