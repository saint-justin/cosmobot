export interface ScrapeRequest {
    GET:        Get;
    success:    Success;
    time_total: string;
}

export interface Get {
    searchtype:      string;
    searchcardstype: string;
}

export interface Success {
    cards: Card[];
}

export interface Card {
    cid:           number;
    name:          string;
    type:          Type;
    cost:          number;
    power:         number;
    ability:       string;
    flavor:        string;
    art:           string;
    alternate_art: string;
    url:           string;
    status:        CardStatus;
    carddefid:     string;
    variants:      Variant[];
    source:        Source;
    source_slug:   SourceSlug;
    tags:          TagElement[];
    rarity:        string;
    rarity_slug:   string;
    difficulty:    string;
    sketcher:      string;
    inker:         CardInker;
    colorist:      Colorist;
}

export enum Colorist {
    Devoted = "Devoted",
    EduardoFrancisco = "Eduardo Francisco",
    Empty = "",
    EricGuerrero = "Eric Guerrero",
    JonboyMeyers = "Jonboy Meyers",
    JuancarlosHenriquez = "Juancarlos Henriquez",
    RyanKinnaird = "Ryan Kinnaird",
}

export enum CardInker {
    Empty = "",
    JasonKang = "Jason Kang",
    JomaroKindred = "Jomaro Kindred",
    RyanBenjamin = "Ryan Benjamin",
}

export enum Source {
    CollectionLevel114 = "Collection Level 1-14",
    CollectionLevel18214Pool1 = "Collection Level 18-214 (Pool 1)",
    CollectionLevel222474Pool2 = "Collection Level 222-474 (Pool 2)",
    CollectionLevel486Pool3 = "Collection Level 486+ (Pool 3)",
    None = "None",
    NotAvailable = "Not Available",
    RecruitSeason = "Recruit Season",
    Series4RareCollectionLevel486Pool4 = "Series 4 Rare - Collection Level 486+ (Pool 4)",
    Series5UltraRareCollectionLevel486Pool5 = "Series 5 Ultra Rare - Collection Level 486+ (Pool 5)",
    StarterCard = "Starter Card",
}

export enum SourceSlug {
    CollectionLevel = "collection-level",
    None = "None",
    NotAvailable = "not-available",
    Pool1 = "pool-1",
    Pool2 = "pool-2",
    Pool3 = "pool-3",
    Pool4 = "pool-4",
    Pool5 = "pool-5",
    RecruitSeason = "recruit-season",
    StarterCard = "starter-card",
}

export enum CardStatus {
    Released = "released",
    Unreleased = "unreleased",
}

export interface TagElement {
    tag_id:   number;
    tag:      TagEnum;
    tag_slug: TagSlug;
}

export enum TagEnum {
    Destroy = "Destroy",
    Discard = "Discard",
    GuardiansOfTheGalaxy = "Guardians of the Galaxy",
    Hydra = "HYDRA",
    Move = "Move",
    NoAbility = "No Ability",
    OnReveal = "On Reveal",
    Ongoing = "Ongoing",
    SHIELD = "S.H.I.E.L.D.",
}

export enum TagSlug {
    Destroy = "destroy",
    Discard = "discard",
    GuardiansOfTheGalaxy = "guardians-of-the-galaxy",
    Hydra = "hydra",
    Move = "move",
    NoAbility = "no-ability",
    OnReveal = "on-reveal",
    Ongoing = "ongoing",
    Shield = "shield",
}

export enum Type {
    Character = "Character",
}

export interface Variant {
    cid:              number;
    vid:              number;
    art:              string;
    art_filename:     string;
    rarity:           Rarity;
    rarity_slug:      RaritySlug;
    variant_order:    string;
    status:           VariantStatus;
    full_description: string;
    inker:            VariantInker;
    sketcher:         string;
    colorist:         string;
    possession:       number | string;
    usage_count:      number | string;
    ReleaseDate:      number | string;
    UseIfOwn:         string;
    PossesionShare:   string;
    UsageShare:       string;
}

export enum VariantInker {
    AndrewHennessy = "Andrew Hennessy",
    CoryHamscher = "Cory Hamscher",
    DannyMiki = "Danny Miki",
    Empty = "",
    GeanesHolland = "Geanes Holland",
    JPMayer = "JP Mayer",
    MarkFarmer = "Mark Farmer",
    MattBanning = "Matt Banning",
    RafaelFonteriz = "Rafael Fonteriz",
    RobertoPoggi = "Roberto Poggi",
    ScottWilliams = "Scott Williams",
    SeanParsons = "Sean Parsons",
    TopCowRickBasaldua = "Top Cow Rick Basaldua",
    WadeVonGrawbadger = "Wade Von Grawbadger",
}

export enum Rarity {
    Bundle = "bundle",
    Conquest = "conquest",
    Empty = "",
    Event = "event",
    Rare = "Rare",
    RarityRare = "rare",
    RaritySuperRare = "super rare",
    SeasonPass = "season pass",
    Spotlight = "spotlight",
    SuperRare = "Super Rare",
    TokenShopUltimate = "Token Shop Ultimate",
}

export enum RaritySlug {
    Bundle = "bundle",
    Conquest = "conquest",
    Empty = "",
    Event = "event",
    Rare = "rare",
    SeasonPass = "season-pass",
    Spotlight = "spotlight",
    SuperRare = "super-rare",
    TokenShopUltimate = "token-shop-ultimate",
}

export enum VariantStatus {
    Released = "Released",
    Unreleased = "Unreleased",
}
