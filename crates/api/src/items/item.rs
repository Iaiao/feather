// This file is @generated. Please do not edit.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde :: Serialize,
    serde :: Deserialize,
)]
#[serde(try_from = "String", into = "&'static str")]
pub enum ItemKind {
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    Deepslate,
    CobbledDeepslate,
    PolishedDeepslate,
    Calcite,
    Tuff,
    DripstoneBlock,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    RootedDirt,
    CrimsonNylium,
    WarpedNylium,
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    CrimsonPlanks,
    WarpedPlanks,
    OakSapling,
    SpruceSapling,
    BirchSapling,
    JungleSapling,
    AcaciaSapling,
    DarkOakSapling,
    Bedrock,
    Sand,
    RedSand,
    Gravel,
    CoalOre,
    DeepslateCoalOre,
    IronOre,
    DeepslateIronOre,
    CopperOre,
    DeepslateCopperOre,
    GoldOre,
    DeepslateGoldOre,
    RedstoneOre,
    DeepslateRedstoneOre,
    EmeraldOre,
    DeepslateEmeraldOre,
    LapisOre,
    DeepslateLapisOre,
    DiamondOre,
    DeepslateDiamondOre,
    NetherGoldOre,
    NetherQuartzOre,
    AncientDebris,
    CoalBlock,
    RawIronBlock,
    RawCopperBlock,
    RawGoldBlock,
    AmethystBlock,
    BuddingAmethyst,
    IronBlock,
    CopperBlock,
    GoldBlock,
    DiamondBlock,
    NetheriteBlock,
    ExposedCopper,
    WeatheredCopper,
    OxidizedCopper,
    CutCopper,
    ExposedCutCopper,
    WeatheredCutCopper,
    OxidizedCutCopper,
    CutCopperStairs,
    ExposedCutCopperStairs,
    WeatheredCutCopperStairs,
    OxidizedCutCopperStairs,
    CutCopperSlab,
    ExposedCutCopperSlab,
    WeatheredCutCopperSlab,
    OxidizedCutCopperSlab,
    WaxedCopperBlock,
    WaxedExposedCopper,
    WaxedWeatheredCopper,
    WaxedOxidizedCopper,
    WaxedCutCopper,
    WaxedExposedCutCopper,
    WaxedWeatheredCutCopper,
    WaxedOxidizedCutCopper,
    WaxedCutCopperStairs,
    WaxedExposedCutCopperStairs,
    WaxedWeatheredCutCopperStairs,
    WaxedOxidizedCutCopperStairs,
    WaxedCutCopperSlab,
    WaxedExposedCutCopperSlab,
    WaxedWeatheredCutCopperSlab,
    WaxedOxidizedCutCopperSlab,
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    AcaciaLog,
    DarkOakLog,
    CrimsonStem,
    WarpedStem,
    StrippedOakLog,
    StrippedSpruceLog,
    StrippedBirchLog,
    StrippedJungleLog,
    StrippedAcaciaLog,
    StrippedDarkOakLog,
    StrippedCrimsonStem,
    StrippedWarpedStem,
    StrippedOakWood,
    StrippedSpruceWood,
    StrippedBirchWood,
    StrippedJungleWood,
    StrippedAcaciaWood,
    StrippedDarkOakWood,
    StrippedCrimsonHyphae,
    StrippedWarpedHyphae,
    OakWood,
    SpruceWood,
    BirchWood,
    JungleWood,
    AcaciaWood,
    DarkOakWood,
    CrimsonHyphae,
    WarpedHyphae,
    OakLeaves,
    SpruceLeaves,
    BirchLeaves,
    JungleLeaves,
    AcaciaLeaves,
    DarkOakLeaves,
    AzaleaLeaves,
    FloweringAzaleaLeaves,
    Sponge,
    WetSponge,
    Glass,
    TintedGlass,
    LapisBlock,
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    Cobweb,
    Grass,
    Fern,
    Azalea,
    FloweringAzalea,
    DeadBush,
    Seagrass,
    SeaPickle,
    WhiteWool,
    OrangeWool,
    MagentaWool,
    LightBlueWool,
    YellowWool,
    LimeWool,
    PinkWool,
    GrayWool,
    LightGrayWool,
    CyanWool,
    PurpleWool,
    BlueWool,
    BrownWool,
    GreenWool,
    RedWool,
    BlackWool,
    Dandelion,
    Poppy,
    BlueOrchid,
    Allium,
    AzureBluet,
    RedTulip,
    OrangeTulip,
    WhiteTulip,
    PinkTulip,
    OxeyeDaisy,
    Cornflower,
    LilyOfTheValley,
    WitherRose,
    SporeBlossom,
    BrownMushroom,
    RedMushroom,
    CrimsonFungus,
    WarpedFungus,
    CrimsonRoots,
    WarpedRoots,
    NetherSprouts,
    WeepingVines,
    TwistingVines,
    SugarCane,
    Kelp,
    MossCarpet,
    MossBlock,
    HangingRoots,
    BigDripleaf,
    SmallDripleaf,
    Bamboo,
    OakSlab,
    SpruceSlab,
    BirchSlab,
    JungleSlab,
    AcaciaSlab,
    DarkOakSlab,
    CrimsonSlab,
    WarpedSlab,
    StoneSlab,
    SmoothStoneSlab,
    SandstoneSlab,
    CutSandstoneSlab,
    PetrifiedOakSlab,
    CobblestoneSlab,
    BrickSlab,
    StoneBrickSlab,
    NetherBrickSlab,
    QuartzSlab,
    RedSandstoneSlab,
    CutRedSandstoneSlab,
    PurpurSlab,
    PrismarineSlab,
    PrismarineBrickSlab,
    DarkPrismarineSlab,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Bricks,
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    Torch,
    EndRod,
    ChorusPlant,
    ChorusFlower,
    PurpurBlock,
    PurpurPillar,
    PurpurStairs,
    Spawner,
    OakStairs,
    Chest,
    CraftingTable,
    Farmland,
    Furnace,
    Ladder,
    CobblestoneStairs,
    Snow,
    Ice,
    SnowBlock,
    Cactus,
    Clay,
    Jukebox,
    OakFence,
    SpruceFence,
    BirchFence,
    JungleFence,
    AcaciaFence,
    DarkOakFence,
    CrimsonFence,
    WarpedFence,
    Pumpkin,
    CarvedPumpkin,
    JackOLantern,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    PolishedBasalt,
    SmoothBasalt,
    SoulTorch,
    Glowstone,
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    InfestedDeepslate,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    DeepslateBricks,
    CrackedDeepslateBricks,
    DeepslateTiles,
    CrackedDeepslateTiles,
    ChiseledDeepslate,
    BrownMushroomBlock,
    RedMushroomBlock,
    MushroomStem,
    IronBars,
    Chain,
    GlassPane,
    Melon,
    Vine,
    GlowLichen,
    BrickStairs,
    StoneBrickStairs,
    Mycelium,
    LilyPad,
    NetherBricks,
    CrackedNetherBricks,
    ChiseledNetherBricks,
    NetherBrickFence,
    NetherBrickStairs,
    EnchantingTable,
    EndPortalFrame,
    EndStone,
    EndStoneBricks,
    DragonEgg,
    SandstoneStairs,
    EnderChest,
    EmeraldBlock,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    CrimsonStairs,
    WarpedStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    MossyCobblestoneWall,
    BrickWall,
    PrismarineWall,
    RedSandstoneWall,
    MossyStoneBrickWall,
    GraniteWall,
    StoneBrickWall,
    NetherBrickWall,
    AndesiteWall,
    RedNetherBrickWall,
    SandstoneWall,
    EndStoneBrickWall,
    DioriteWall,
    BlackstoneWall,
    PolishedBlackstoneWall,
    PolishedBlackstoneBrickWall,
    CobbledDeepslateWall,
    PolishedDeepslateWall,
    DeepslateBrickWall,
    DeepslateTileWall,
    Anvil,
    ChippedAnvil,
    DamagedAnvil,
    ChiseledQuartzBlock,
    QuartzBlock,
    QuartzBricks,
    QuartzPillar,
    QuartzStairs,
    WhiteTerracotta,
    OrangeTerracotta,
    MagentaTerracotta,
    LightBlueTerracotta,
    YellowTerracotta,
    LimeTerracotta,
    PinkTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    CyanTerracotta,
    PurpleTerracotta,
    BlueTerracotta,
    BrownTerracotta,
    GreenTerracotta,
    RedTerracotta,
    BlackTerracotta,
    Barrier,
    Light,
    HayBlock,
    WhiteCarpet,
    OrangeCarpet,
    MagentaCarpet,
    LightBlueCarpet,
    YellowCarpet,
    LimeCarpet,
    PinkCarpet,
    GrayCarpet,
    LightGrayCarpet,
    CyanCarpet,
    PurpleCarpet,
    BlueCarpet,
    BrownCarpet,
    GreenCarpet,
    RedCarpet,
    BlackCarpet,
    Terracotta,
    PackedIce,
    AcaciaStairs,
    DarkOakStairs,
    DirtPath,
    Sunflower,
    Lilac,
    RoseBush,
    Peony,
    TallGrass,
    LargeFern,
    WhiteStainedGlass,
    OrangeStainedGlass,
    MagentaStainedGlass,
    LightBlueStainedGlass,
    YellowStainedGlass,
    LimeStainedGlass,
    PinkStainedGlass,
    GrayStainedGlass,
    LightGrayStainedGlass,
    CyanStainedGlass,
    PurpleStainedGlass,
    BlueStainedGlass,
    BrownStainedGlass,
    GreenStainedGlass,
    RedStainedGlass,
    BlackStainedGlass,
    WhiteStainedGlassPane,
    OrangeStainedGlassPane,
    MagentaStainedGlassPane,
    LightBlueStainedGlassPane,
    YellowStainedGlassPane,
    LimeStainedGlassPane,
    PinkStainedGlassPane,
    GrayStainedGlassPane,
    LightGrayStainedGlassPane,
    CyanStainedGlassPane,
    PurpleStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    GreenStainedGlassPane,
    RedStainedGlassPane,
    BlackStainedGlassPane,
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs,
    PrismarineBrickStairs,
    DarkPrismarineStairs,
    SeaLantern,
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs,
    RepeatingCommandBlock,
    ChainCommandBlock,
    MagmaBlock,
    NetherWartBlock,
    WarpedWartBlock,
    RedNetherBricks,
    BoneBlock,
    StructureVoid,
    ShulkerBox,
    WhiteShulkerBox,
    OrangeShulkerBox,
    MagentaShulkerBox,
    LightBlueShulkerBox,
    YellowShulkerBox,
    LimeShulkerBox,
    PinkShulkerBox,
    GrayShulkerBox,
    LightGrayShulkerBox,
    CyanShulkerBox,
    PurpleShulkerBox,
    BlueShulkerBox,
    BrownShulkerBox,
    GreenShulkerBox,
    RedShulkerBox,
    BlackShulkerBox,
    WhiteGlazedTerracotta,
    OrangeGlazedTerracotta,
    MagentaGlazedTerracotta,
    LightBlueGlazedTerracotta,
    YellowGlazedTerracotta,
    LimeGlazedTerracotta,
    PinkGlazedTerracotta,
    GrayGlazedTerracotta,
    LightGrayGlazedTerracotta,
    CyanGlazedTerracotta,
    PurpleGlazedTerracotta,
    BlueGlazedTerracotta,
    BrownGlazedTerracotta,
    GreenGlazedTerracotta,
    RedGlazedTerracotta,
    BlackGlazedTerracotta,
    WhiteConcrete,
    OrangeConcrete,
    MagentaConcrete,
    LightBlueConcrete,
    YellowConcrete,
    LimeConcrete,
    PinkConcrete,
    GrayConcrete,
    LightGrayConcrete,
    CyanConcrete,
    PurpleConcrete,
    BlueConcrete,
    BrownConcrete,
    GreenConcrete,
    RedConcrete,
    BlackConcrete,
    WhiteConcretePowder,
    OrangeConcretePowder,
    MagentaConcretePowder,
    LightBlueConcretePowder,
    YellowConcretePowder,
    LimeConcretePowder,
    PinkConcretePowder,
    GrayConcretePowder,
    LightGrayConcretePowder,
    CyanConcretePowder,
    PurpleConcretePowder,
    BlueConcretePowder,
    BrownConcretePowder,
    GreenConcretePowder,
    RedConcretePowder,
    BlackConcretePowder,
    TurtleEgg,
    DeadTubeCoralBlock,
    DeadBrainCoralBlock,
    DeadBubbleCoralBlock,
    DeadFireCoralBlock,
    DeadHornCoralBlock,
    TubeCoralBlock,
    BrainCoralBlock,
    BubbleCoralBlock,
    FireCoralBlock,
    HornCoralBlock,
    TubeCoral,
    BrainCoral,
    BubbleCoral,
    FireCoral,
    HornCoral,
    DeadBrainCoral,
    DeadBubbleCoral,
    DeadFireCoral,
    DeadHornCoral,
    DeadTubeCoral,
    TubeCoralFan,
    BrainCoralFan,
    BubbleCoralFan,
    FireCoralFan,
    HornCoralFan,
    DeadTubeCoralFan,
    DeadBrainCoralFan,
    DeadBubbleCoralFan,
    DeadFireCoralFan,
    DeadHornCoralFan,
    BlueIce,
    Conduit,
    PolishedGraniteStairs,
    SmoothRedSandstoneStairs,
    MossyStoneBrickStairs,
    PolishedDioriteStairs,
    MossyCobblestoneStairs,
    EndStoneBrickStairs,
    StoneStairs,
    SmoothSandstoneStairs,
    SmoothQuartzStairs,
    GraniteStairs,
    AndesiteStairs,
    RedNetherBrickStairs,
    PolishedAndesiteStairs,
    DioriteStairs,
    CobbledDeepslateStairs,
    PolishedDeepslateStairs,
    DeepslateBrickStairs,
    DeepslateTileStairs,
    PolishedGraniteSlab,
    SmoothRedSandstoneSlab,
    MossyStoneBrickSlab,
    PolishedDioriteSlab,
    MossyCobblestoneSlab,
    EndStoneBrickSlab,
    SmoothSandstoneSlab,
    SmoothQuartzSlab,
    GraniteSlab,
    AndesiteSlab,
    RedNetherBrickSlab,
    PolishedAndesiteSlab,
    DioriteSlab,
    CobbledDeepslateSlab,
    PolishedDeepslateSlab,
    DeepslateBrickSlab,
    DeepslateTileSlab,
    Scaffolding,
    Redstone,
    RedstoneTorch,
    RedstoneBlock,
    Repeater,
    Comparator,
    Piston,
    StickyPiston,
    SlimeBlock,
    HoneyBlock,
    Observer,
    Hopper,
    Dispenser,
    Dropper,
    Lectern,
    Target,
    Lever,
    LightningRod,
    DaylightDetector,
    SculkSensor,
    TripwireHook,
    TrappedChest,
    Tnt,
    RedstoneLamp,
    NoteBlock,
    StoneButton,
    PolishedBlackstoneButton,
    OakButton,
    SpruceButton,
    BirchButton,
    JungleButton,
    AcaciaButton,
    DarkOakButton,
    CrimsonButton,
    WarpedButton,
    StonePressurePlate,
    PolishedBlackstonePressurePlate,
    LightWeightedPressurePlate,
    HeavyWeightedPressurePlate,
    OakPressurePlate,
    SprucePressurePlate,
    BirchPressurePlate,
    JunglePressurePlate,
    AcaciaPressurePlate,
    DarkOakPressurePlate,
    CrimsonPressurePlate,
    WarpedPressurePlate,
    IronDoor,
    OakDoor,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    DarkOakDoor,
    CrimsonDoor,
    WarpedDoor,
    IronTrapdoor,
    OakTrapdoor,
    SpruceTrapdoor,
    BirchTrapdoor,
    JungleTrapdoor,
    AcaciaTrapdoor,
    DarkOakTrapdoor,
    CrimsonTrapdoor,
    WarpedTrapdoor,
    OakFenceGate,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    AcaciaFenceGate,
    DarkOakFenceGate,
    CrimsonFenceGate,
    WarpedFenceGate,
    PoweredRail,
    DetectorRail,
    Rail,
    ActivatorRail,
    Saddle,
    Minecart,
    ChestMinecart,
    FurnaceMinecart,
    TntMinecart,
    HopperMinecart,
    CarrotOnAStick,
    WarpedFungusOnAStick,
    Elytra,
    OakBoat,
    SpruceBoat,
    BirchBoat,
    JungleBoat,
    AcaciaBoat,
    DarkOakBoat,
    StructureBlock,
    Jigsaw,
    TurtleHelmet,
    Scute,
    FlintAndSteel,
    Apple,
    Bow,
    Arrow,
    Coal,
    Charcoal,
    Diamond,
    Emerald,
    LapisLazuli,
    Quartz,
    AmethystShard,
    RawIron,
    IronIngot,
    RawCopper,
    CopperIngot,
    RawGold,
    GoldIngot,
    NetheriteIngot,
    NetheriteScrap,
    WoodenSword,
    WoodenShovel,
    WoodenPickaxe,
    WoodenAxe,
    WoodenHoe,
    StoneSword,
    StoneShovel,
    StonePickaxe,
    StoneAxe,
    StoneHoe,
    GoldenSword,
    GoldenShovel,
    GoldenPickaxe,
    GoldenAxe,
    GoldenHoe,
    IronSword,
    IronShovel,
    IronPickaxe,
    IronAxe,
    IronHoe,
    DiamondSword,
    DiamondShovel,
    DiamondPickaxe,
    DiamondAxe,
    DiamondHoe,
    NetheriteSword,
    NetheriteShovel,
    NetheritePickaxe,
    NetheriteAxe,
    NetheriteHoe,
    Stick,
    Bowl,
    MushroomStew,
    String,
    Feather,
    Gunpowder,
    WheatSeeds,
    Wheat,
    Bread,
    LeatherHelmet,
    LeatherChestplate,
    LeatherLeggings,
    LeatherBoots,
    ChainmailHelmet,
    ChainmailChestplate,
    ChainmailLeggings,
    ChainmailBoots,
    IronHelmet,
    IronChestplate,
    IronLeggings,
    IronBoots,
    DiamondHelmet,
    DiamondChestplate,
    DiamondLeggings,
    DiamondBoots,
    GoldenHelmet,
    GoldenChestplate,
    GoldenLeggings,
    GoldenBoots,
    NetheriteHelmet,
    NetheriteChestplate,
    NetheriteLeggings,
    NetheriteBoots,
    Flint,
    Porkchop,
    CookedPorkchop,
    Painting,
    GoldenApple,
    EnchantedGoldenApple,
    OakSign,
    SpruceSign,
    BirchSign,
    JungleSign,
    AcaciaSign,
    DarkOakSign,
    CrimsonSign,
    WarpedSign,
    Bucket,
    WaterBucket,
    LavaBucket,
    PowderSnowBucket,
    Snowball,
    Leather,
    MilkBucket,
    PufferfishBucket,
    SalmonBucket,
    CodBucket,
    TropicalFishBucket,
    AxolotlBucket,
    Brick,
    ClayBall,
    DriedKelpBlock,
    Paper,
    Book,
    SlimeBall,
    Egg,
    Compass,
    Bundle,
    FishingRod,
    Clock,
    Spyglass,
    GlowstoneDust,
    Cod,
    Salmon,
    TropicalFish,
    Pufferfish,
    CookedCod,
    CookedSalmon,
    InkSac,
    GlowInkSac,
    CocoaBeans,
    WhiteDye,
    OrangeDye,
    MagentaDye,
    LightBlueDye,
    YellowDye,
    LimeDye,
    PinkDye,
    GrayDye,
    LightGrayDye,
    CyanDye,
    PurpleDye,
    BlueDye,
    BrownDye,
    GreenDye,
    RedDye,
    BlackDye,
    BoneMeal,
    Bone,
    Sugar,
    Cake,
    WhiteBed,
    OrangeBed,
    MagentaBed,
    LightBlueBed,
    YellowBed,
    LimeBed,
    PinkBed,
    GrayBed,
    LightGrayBed,
    CyanBed,
    PurpleBed,
    BlueBed,
    BrownBed,
    GreenBed,
    RedBed,
    BlackBed,
    Cookie,
    FilledMap,
    Shears,
    MelonSlice,
    DriedKelp,
    PumpkinSeeds,
    MelonSeeds,
    Beef,
    CookedBeef,
    Chicken,
    CookedChicken,
    RottenFlesh,
    EnderPearl,
    BlazeRod,
    GhastTear,
    GoldNugget,
    NetherWart,
    Potion,
    GlassBottle,
    SpiderEye,
    FermentedSpiderEye,
    BlazePowder,
    MagmaCream,
    BrewingStand,
    Cauldron,
    EnderEye,
    GlisteringMelonSlice,
    AxolotlSpawnEgg,
    BatSpawnEgg,
    BeeSpawnEgg,
    BlazeSpawnEgg,
    CatSpawnEgg,
    CaveSpiderSpawnEgg,
    ChickenSpawnEgg,
    CodSpawnEgg,
    CowSpawnEgg,
    CreeperSpawnEgg,
    DolphinSpawnEgg,
    DonkeySpawnEgg,
    DrownedSpawnEgg,
    ElderGuardianSpawnEgg,
    EndermanSpawnEgg,
    EndermiteSpawnEgg,
    EvokerSpawnEgg,
    FoxSpawnEgg,
    GhastSpawnEgg,
    GlowSquidSpawnEgg,
    GoatSpawnEgg,
    GuardianSpawnEgg,
    HoglinSpawnEgg,
    HorseSpawnEgg,
    HuskSpawnEgg,
    LlamaSpawnEgg,
    MagmaCubeSpawnEgg,
    MooshroomSpawnEgg,
    MuleSpawnEgg,
    OcelotSpawnEgg,
    PandaSpawnEgg,
    ParrotSpawnEgg,
    PhantomSpawnEgg,
    PigSpawnEgg,
    PiglinSpawnEgg,
    PiglinBruteSpawnEgg,
    PillagerSpawnEgg,
    PolarBearSpawnEgg,
    PufferfishSpawnEgg,
    RabbitSpawnEgg,
    RavagerSpawnEgg,
    SalmonSpawnEgg,
    SheepSpawnEgg,
    ShulkerSpawnEgg,
    SilverfishSpawnEgg,
    SkeletonSpawnEgg,
    SkeletonHorseSpawnEgg,
    SlimeSpawnEgg,
    SpiderSpawnEgg,
    SquidSpawnEgg,
    StraySpawnEgg,
    StriderSpawnEgg,
    TraderLlamaSpawnEgg,
    TropicalFishSpawnEgg,
    TurtleSpawnEgg,
    VexSpawnEgg,
    VillagerSpawnEgg,
    VindicatorSpawnEgg,
    WanderingTraderSpawnEgg,
    WitchSpawnEgg,
    WitherSkeletonSpawnEgg,
    WolfSpawnEgg,
    ZoglinSpawnEgg,
    ZombieSpawnEgg,
    ZombieHorseSpawnEgg,
    ZombieVillagerSpawnEgg,
    ZombifiedPiglinSpawnEgg,
    ExperienceBottle,
    FireCharge,
    WritableBook,
    WrittenBook,
    ItemFrame,
    GlowItemFrame,
    FlowerPot,
    Carrot,
    Potato,
    BakedPotato,
    PoisonousPotato,
    Map,
    GoldenCarrot,
    SkeletonSkull,
    WitherSkeletonSkull,
    PlayerHead,
    ZombieHead,
    CreeperHead,
    DragonHead,
    NetherStar,
    PumpkinPie,
    FireworkRocket,
    FireworkStar,
    EnchantedBook,
    NetherBrick,
    PrismarineShard,
    PrismarineCrystals,
    Rabbit,
    CookedRabbit,
    RabbitStew,
    RabbitFoot,
    RabbitHide,
    ArmorStand,
    IronHorseArmor,
    GoldenHorseArmor,
    DiamondHorseArmor,
    LeatherHorseArmor,
    Lead,
    NameTag,
    CommandBlockMinecart,
    Mutton,
    CookedMutton,
    WhiteBanner,
    OrangeBanner,
    MagentaBanner,
    LightBlueBanner,
    YellowBanner,
    LimeBanner,
    PinkBanner,
    GrayBanner,
    LightGrayBanner,
    CyanBanner,
    PurpleBanner,
    BlueBanner,
    BrownBanner,
    GreenBanner,
    RedBanner,
    BlackBanner,
    EndCrystal,
    ChorusFruit,
    PoppedChorusFruit,
    Beetroot,
    BeetrootSeeds,
    BeetrootSoup,
    DragonBreath,
    SplashPotion,
    SpectralArrow,
    TippedArrow,
    LingeringPotion,
    Shield,
    TotemOfUndying,
    ShulkerShell,
    IronNugget,
    KnowledgeBook,
    DebugStick,
    MusicDisc13,
    MusicDiscCat,
    MusicDiscBlocks,
    MusicDiscChirp,
    MusicDiscFar,
    MusicDiscMall,
    MusicDiscMellohi,
    MusicDiscStal,
    MusicDiscStrad,
    MusicDiscWard,
    MusicDisc11,
    MusicDiscWait,
    MusicDiscOtherside,
    MusicDiscPigstep,
    Trident,
    PhantomMembrane,
    NautilusShell,
    HeartOfTheSea,
    Crossbow,
    SuspiciousStew,
    Loom,
    FlowerBannerPattern,
    CreeperBannerPattern,
    SkullBannerPattern,
    MojangBannerPattern,
    GlobeBannerPattern,
    PiglinBannerPattern,
    Composter,
    Barrel,
    Smoker,
    BlastFurnace,
    CartographyTable,
    FletchingTable,
    Grindstone,
    SmithingTable,
    Stonecutter,
    Bell,
    Lantern,
    SoulLantern,
    SweetBerries,
    GlowBerries,
    Campfire,
    SoulCampfire,
    Shroomlight,
    Honeycomb,
    BeeNest,
    Beehive,
    HoneyBottle,
    HoneycombBlock,
    Lodestone,
    CryingObsidian,
    Blackstone,
    BlackstoneSlab,
    BlackstoneStairs,
    GildedBlackstone,
    PolishedBlackstone,
    PolishedBlackstoneSlab,
    PolishedBlackstoneStairs,
    ChiseledPolishedBlackstone,
    PolishedBlackstoneBricks,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneBrickStairs,
    CrackedPolishedBlackstoneBricks,
    RespawnAnchor,
    Candle,
    WhiteCandle,
    OrangeCandle,
    MagentaCandle,
    LightBlueCandle,
    YellowCandle,
    LimeCandle,
    PinkCandle,
    GrayCandle,
    LightGrayCandle,
    CyanCandle,
    PurpleCandle,
    BlueCandle,
    BrownCandle,
    GreenCandle,
    RedCandle,
    BlackCandle,
    SmallAmethystBud,
    MediumAmethystBud,
    LargeAmethystBud,
    AmethystCluster,
    PointedDripstone,
}
impl ItemKind {
    #[inline]
    pub fn values() -> &'static [ItemKind] {
        use ItemKind::*;
        &[
            Stone,
            Granite,
            PolishedGranite,
            Diorite,
            PolishedDiorite,
            Andesite,
            PolishedAndesite,
            Deepslate,
            CobbledDeepslate,
            PolishedDeepslate,
            Calcite,
            Tuff,
            DripstoneBlock,
            GrassBlock,
            Dirt,
            CoarseDirt,
            Podzol,
            RootedDirt,
            CrimsonNylium,
            WarpedNylium,
            Cobblestone,
            OakPlanks,
            SprucePlanks,
            BirchPlanks,
            JunglePlanks,
            AcaciaPlanks,
            DarkOakPlanks,
            CrimsonPlanks,
            WarpedPlanks,
            OakSapling,
            SpruceSapling,
            BirchSapling,
            JungleSapling,
            AcaciaSapling,
            DarkOakSapling,
            Bedrock,
            Sand,
            RedSand,
            Gravel,
            CoalOre,
            DeepslateCoalOre,
            IronOre,
            DeepslateIronOre,
            CopperOre,
            DeepslateCopperOre,
            GoldOre,
            DeepslateGoldOre,
            RedstoneOre,
            DeepslateRedstoneOre,
            EmeraldOre,
            DeepslateEmeraldOre,
            LapisOre,
            DeepslateLapisOre,
            DiamondOre,
            DeepslateDiamondOre,
            NetherGoldOre,
            NetherQuartzOre,
            AncientDebris,
            CoalBlock,
            RawIronBlock,
            RawCopperBlock,
            RawGoldBlock,
            AmethystBlock,
            BuddingAmethyst,
            IronBlock,
            CopperBlock,
            GoldBlock,
            DiamondBlock,
            NetheriteBlock,
            ExposedCopper,
            WeatheredCopper,
            OxidizedCopper,
            CutCopper,
            ExposedCutCopper,
            WeatheredCutCopper,
            OxidizedCutCopper,
            CutCopperStairs,
            ExposedCutCopperStairs,
            WeatheredCutCopperStairs,
            OxidizedCutCopperStairs,
            CutCopperSlab,
            ExposedCutCopperSlab,
            WeatheredCutCopperSlab,
            OxidizedCutCopperSlab,
            WaxedCopperBlock,
            WaxedExposedCopper,
            WaxedWeatheredCopper,
            WaxedOxidizedCopper,
            WaxedCutCopper,
            WaxedExposedCutCopper,
            WaxedWeatheredCutCopper,
            WaxedOxidizedCutCopper,
            WaxedCutCopperStairs,
            WaxedExposedCutCopperStairs,
            WaxedWeatheredCutCopperStairs,
            WaxedOxidizedCutCopperStairs,
            WaxedCutCopperSlab,
            WaxedExposedCutCopperSlab,
            WaxedWeatheredCutCopperSlab,
            WaxedOxidizedCutCopperSlab,
            OakLog,
            SpruceLog,
            BirchLog,
            JungleLog,
            AcaciaLog,
            DarkOakLog,
            CrimsonStem,
            WarpedStem,
            StrippedOakLog,
            StrippedSpruceLog,
            StrippedBirchLog,
            StrippedJungleLog,
            StrippedAcaciaLog,
            StrippedDarkOakLog,
            StrippedCrimsonStem,
            StrippedWarpedStem,
            StrippedOakWood,
            StrippedSpruceWood,
            StrippedBirchWood,
            StrippedJungleWood,
            StrippedAcaciaWood,
            StrippedDarkOakWood,
            StrippedCrimsonHyphae,
            StrippedWarpedHyphae,
            OakWood,
            SpruceWood,
            BirchWood,
            JungleWood,
            AcaciaWood,
            DarkOakWood,
            CrimsonHyphae,
            WarpedHyphae,
            OakLeaves,
            SpruceLeaves,
            BirchLeaves,
            JungleLeaves,
            AcaciaLeaves,
            DarkOakLeaves,
            AzaleaLeaves,
            FloweringAzaleaLeaves,
            Sponge,
            WetSponge,
            Glass,
            TintedGlass,
            LapisBlock,
            Sandstone,
            ChiseledSandstone,
            CutSandstone,
            Cobweb,
            Grass,
            Fern,
            Azalea,
            FloweringAzalea,
            DeadBush,
            Seagrass,
            SeaPickle,
            WhiteWool,
            OrangeWool,
            MagentaWool,
            LightBlueWool,
            YellowWool,
            LimeWool,
            PinkWool,
            GrayWool,
            LightGrayWool,
            CyanWool,
            PurpleWool,
            BlueWool,
            BrownWool,
            GreenWool,
            RedWool,
            BlackWool,
            Dandelion,
            Poppy,
            BlueOrchid,
            Allium,
            AzureBluet,
            RedTulip,
            OrangeTulip,
            WhiteTulip,
            PinkTulip,
            OxeyeDaisy,
            Cornflower,
            LilyOfTheValley,
            WitherRose,
            SporeBlossom,
            BrownMushroom,
            RedMushroom,
            CrimsonFungus,
            WarpedFungus,
            CrimsonRoots,
            WarpedRoots,
            NetherSprouts,
            WeepingVines,
            TwistingVines,
            SugarCane,
            Kelp,
            MossCarpet,
            MossBlock,
            HangingRoots,
            BigDripleaf,
            SmallDripleaf,
            Bamboo,
            OakSlab,
            SpruceSlab,
            BirchSlab,
            JungleSlab,
            AcaciaSlab,
            DarkOakSlab,
            CrimsonSlab,
            WarpedSlab,
            StoneSlab,
            SmoothStoneSlab,
            SandstoneSlab,
            CutSandstoneSlab,
            PetrifiedOakSlab,
            CobblestoneSlab,
            BrickSlab,
            StoneBrickSlab,
            NetherBrickSlab,
            QuartzSlab,
            RedSandstoneSlab,
            CutRedSandstoneSlab,
            PurpurSlab,
            PrismarineSlab,
            PrismarineBrickSlab,
            DarkPrismarineSlab,
            SmoothQuartz,
            SmoothRedSandstone,
            SmoothSandstone,
            SmoothStone,
            Bricks,
            Bookshelf,
            MossyCobblestone,
            Obsidian,
            Torch,
            EndRod,
            ChorusPlant,
            ChorusFlower,
            PurpurBlock,
            PurpurPillar,
            PurpurStairs,
            Spawner,
            OakStairs,
            Chest,
            CraftingTable,
            Farmland,
            Furnace,
            Ladder,
            CobblestoneStairs,
            Snow,
            Ice,
            SnowBlock,
            Cactus,
            Clay,
            Jukebox,
            OakFence,
            SpruceFence,
            BirchFence,
            JungleFence,
            AcaciaFence,
            DarkOakFence,
            CrimsonFence,
            WarpedFence,
            Pumpkin,
            CarvedPumpkin,
            JackOLantern,
            Netherrack,
            SoulSand,
            SoulSoil,
            Basalt,
            PolishedBasalt,
            SmoothBasalt,
            SoulTorch,
            Glowstone,
            InfestedStone,
            InfestedCobblestone,
            InfestedStoneBricks,
            InfestedMossyStoneBricks,
            InfestedCrackedStoneBricks,
            InfestedChiseledStoneBricks,
            InfestedDeepslate,
            StoneBricks,
            MossyStoneBricks,
            CrackedStoneBricks,
            ChiseledStoneBricks,
            DeepslateBricks,
            CrackedDeepslateBricks,
            DeepslateTiles,
            CrackedDeepslateTiles,
            ChiseledDeepslate,
            BrownMushroomBlock,
            RedMushroomBlock,
            MushroomStem,
            IronBars,
            Chain,
            GlassPane,
            Melon,
            Vine,
            GlowLichen,
            BrickStairs,
            StoneBrickStairs,
            Mycelium,
            LilyPad,
            NetherBricks,
            CrackedNetherBricks,
            ChiseledNetherBricks,
            NetherBrickFence,
            NetherBrickStairs,
            EnchantingTable,
            EndPortalFrame,
            EndStone,
            EndStoneBricks,
            DragonEgg,
            SandstoneStairs,
            EnderChest,
            EmeraldBlock,
            SpruceStairs,
            BirchStairs,
            JungleStairs,
            CrimsonStairs,
            WarpedStairs,
            CommandBlock,
            Beacon,
            CobblestoneWall,
            MossyCobblestoneWall,
            BrickWall,
            PrismarineWall,
            RedSandstoneWall,
            MossyStoneBrickWall,
            GraniteWall,
            StoneBrickWall,
            NetherBrickWall,
            AndesiteWall,
            RedNetherBrickWall,
            SandstoneWall,
            EndStoneBrickWall,
            DioriteWall,
            BlackstoneWall,
            PolishedBlackstoneWall,
            PolishedBlackstoneBrickWall,
            CobbledDeepslateWall,
            PolishedDeepslateWall,
            DeepslateBrickWall,
            DeepslateTileWall,
            Anvil,
            ChippedAnvil,
            DamagedAnvil,
            ChiseledQuartzBlock,
            QuartzBlock,
            QuartzBricks,
            QuartzPillar,
            QuartzStairs,
            WhiteTerracotta,
            OrangeTerracotta,
            MagentaTerracotta,
            LightBlueTerracotta,
            YellowTerracotta,
            LimeTerracotta,
            PinkTerracotta,
            GrayTerracotta,
            LightGrayTerracotta,
            CyanTerracotta,
            PurpleTerracotta,
            BlueTerracotta,
            BrownTerracotta,
            GreenTerracotta,
            RedTerracotta,
            BlackTerracotta,
            Barrier,
            Light,
            HayBlock,
            WhiteCarpet,
            OrangeCarpet,
            MagentaCarpet,
            LightBlueCarpet,
            YellowCarpet,
            LimeCarpet,
            PinkCarpet,
            GrayCarpet,
            LightGrayCarpet,
            CyanCarpet,
            PurpleCarpet,
            BlueCarpet,
            BrownCarpet,
            GreenCarpet,
            RedCarpet,
            BlackCarpet,
            Terracotta,
            PackedIce,
            AcaciaStairs,
            DarkOakStairs,
            DirtPath,
            Sunflower,
            Lilac,
            RoseBush,
            Peony,
            TallGrass,
            LargeFern,
            WhiteStainedGlass,
            OrangeStainedGlass,
            MagentaStainedGlass,
            LightBlueStainedGlass,
            YellowStainedGlass,
            LimeStainedGlass,
            PinkStainedGlass,
            GrayStainedGlass,
            LightGrayStainedGlass,
            CyanStainedGlass,
            PurpleStainedGlass,
            BlueStainedGlass,
            BrownStainedGlass,
            GreenStainedGlass,
            RedStainedGlass,
            BlackStainedGlass,
            WhiteStainedGlassPane,
            OrangeStainedGlassPane,
            MagentaStainedGlassPane,
            LightBlueStainedGlassPane,
            YellowStainedGlassPane,
            LimeStainedGlassPane,
            PinkStainedGlassPane,
            GrayStainedGlassPane,
            LightGrayStainedGlassPane,
            CyanStainedGlassPane,
            PurpleStainedGlassPane,
            BlueStainedGlassPane,
            BrownStainedGlassPane,
            GreenStainedGlassPane,
            RedStainedGlassPane,
            BlackStainedGlassPane,
            Prismarine,
            PrismarineBricks,
            DarkPrismarine,
            PrismarineStairs,
            PrismarineBrickStairs,
            DarkPrismarineStairs,
            SeaLantern,
            RedSandstone,
            ChiseledRedSandstone,
            CutRedSandstone,
            RedSandstoneStairs,
            RepeatingCommandBlock,
            ChainCommandBlock,
            MagmaBlock,
            NetherWartBlock,
            WarpedWartBlock,
            RedNetherBricks,
            BoneBlock,
            StructureVoid,
            ShulkerBox,
            WhiteShulkerBox,
            OrangeShulkerBox,
            MagentaShulkerBox,
            LightBlueShulkerBox,
            YellowShulkerBox,
            LimeShulkerBox,
            PinkShulkerBox,
            GrayShulkerBox,
            LightGrayShulkerBox,
            CyanShulkerBox,
            PurpleShulkerBox,
            BlueShulkerBox,
            BrownShulkerBox,
            GreenShulkerBox,
            RedShulkerBox,
            BlackShulkerBox,
            WhiteGlazedTerracotta,
            OrangeGlazedTerracotta,
            MagentaGlazedTerracotta,
            LightBlueGlazedTerracotta,
            YellowGlazedTerracotta,
            LimeGlazedTerracotta,
            PinkGlazedTerracotta,
            GrayGlazedTerracotta,
            LightGrayGlazedTerracotta,
            CyanGlazedTerracotta,
            PurpleGlazedTerracotta,
            BlueGlazedTerracotta,
            BrownGlazedTerracotta,
            GreenGlazedTerracotta,
            RedGlazedTerracotta,
            BlackGlazedTerracotta,
            WhiteConcrete,
            OrangeConcrete,
            MagentaConcrete,
            LightBlueConcrete,
            YellowConcrete,
            LimeConcrete,
            PinkConcrete,
            GrayConcrete,
            LightGrayConcrete,
            CyanConcrete,
            PurpleConcrete,
            BlueConcrete,
            BrownConcrete,
            GreenConcrete,
            RedConcrete,
            BlackConcrete,
            WhiteConcretePowder,
            OrangeConcretePowder,
            MagentaConcretePowder,
            LightBlueConcretePowder,
            YellowConcretePowder,
            LimeConcretePowder,
            PinkConcretePowder,
            GrayConcretePowder,
            LightGrayConcretePowder,
            CyanConcretePowder,
            PurpleConcretePowder,
            BlueConcretePowder,
            BrownConcretePowder,
            GreenConcretePowder,
            RedConcretePowder,
            BlackConcretePowder,
            TurtleEgg,
            DeadTubeCoralBlock,
            DeadBrainCoralBlock,
            DeadBubbleCoralBlock,
            DeadFireCoralBlock,
            DeadHornCoralBlock,
            TubeCoralBlock,
            BrainCoralBlock,
            BubbleCoralBlock,
            FireCoralBlock,
            HornCoralBlock,
            TubeCoral,
            BrainCoral,
            BubbleCoral,
            FireCoral,
            HornCoral,
            DeadBrainCoral,
            DeadBubbleCoral,
            DeadFireCoral,
            DeadHornCoral,
            DeadTubeCoral,
            TubeCoralFan,
            BrainCoralFan,
            BubbleCoralFan,
            FireCoralFan,
            HornCoralFan,
            DeadTubeCoralFan,
            DeadBrainCoralFan,
            DeadBubbleCoralFan,
            DeadFireCoralFan,
            DeadHornCoralFan,
            BlueIce,
            Conduit,
            PolishedGraniteStairs,
            SmoothRedSandstoneStairs,
            MossyStoneBrickStairs,
            PolishedDioriteStairs,
            MossyCobblestoneStairs,
            EndStoneBrickStairs,
            StoneStairs,
            SmoothSandstoneStairs,
            SmoothQuartzStairs,
            GraniteStairs,
            AndesiteStairs,
            RedNetherBrickStairs,
            PolishedAndesiteStairs,
            DioriteStairs,
            CobbledDeepslateStairs,
            PolishedDeepslateStairs,
            DeepslateBrickStairs,
            DeepslateTileStairs,
            PolishedGraniteSlab,
            SmoothRedSandstoneSlab,
            MossyStoneBrickSlab,
            PolishedDioriteSlab,
            MossyCobblestoneSlab,
            EndStoneBrickSlab,
            SmoothSandstoneSlab,
            SmoothQuartzSlab,
            GraniteSlab,
            AndesiteSlab,
            RedNetherBrickSlab,
            PolishedAndesiteSlab,
            DioriteSlab,
            CobbledDeepslateSlab,
            PolishedDeepslateSlab,
            DeepslateBrickSlab,
            DeepslateTileSlab,
            Scaffolding,
            Redstone,
            RedstoneTorch,
            RedstoneBlock,
            Repeater,
            Comparator,
            Piston,
            StickyPiston,
            SlimeBlock,
            HoneyBlock,
            Observer,
            Hopper,
            Dispenser,
            Dropper,
            Lectern,
            Target,
            Lever,
            LightningRod,
            DaylightDetector,
            SculkSensor,
            TripwireHook,
            TrappedChest,
            Tnt,
            RedstoneLamp,
            NoteBlock,
            StoneButton,
            PolishedBlackstoneButton,
            OakButton,
            SpruceButton,
            BirchButton,
            JungleButton,
            AcaciaButton,
            DarkOakButton,
            CrimsonButton,
            WarpedButton,
            StonePressurePlate,
            PolishedBlackstonePressurePlate,
            LightWeightedPressurePlate,
            HeavyWeightedPressurePlate,
            OakPressurePlate,
            SprucePressurePlate,
            BirchPressurePlate,
            JunglePressurePlate,
            AcaciaPressurePlate,
            DarkOakPressurePlate,
            CrimsonPressurePlate,
            WarpedPressurePlate,
            IronDoor,
            OakDoor,
            SpruceDoor,
            BirchDoor,
            JungleDoor,
            AcaciaDoor,
            DarkOakDoor,
            CrimsonDoor,
            WarpedDoor,
            IronTrapdoor,
            OakTrapdoor,
            SpruceTrapdoor,
            BirchTrapdoor,
            JungleTrapdoor,
            AcaciaTrapdoor,
            DarkOakTrapdoor,
            CrimsonTrapdoor,
            WarpedTrapdoor,
            OakFenceGate,
            SpruceFenceGate,
            BirchFenceGate,
            JungleFenceGate,
            AcaciaFenceGate,
            DarkOakFenceGate,
            CrimsonFenceGate,
            WarpedFenceGate,
            PoweredRail,
            DetectorRail,
            Rail,
            ActivatorRail,
            Saddle,
            Minecart,
            ChestMinecart,
            FurnaceMinecart,
            TntMinecart,
            HopperMinecart,
            CarrotOnAStick,
            WarpedFungusOnAStick,
            Elytra,
            OakBoat,
            SpruceBoat,
            BirchBoat,
            JungleBoat,
            AcaciaBoat,
            DarkOakBoat,
            StructureBlock,
            Jigsaw,
            TurtleHelmet,
            Scute,
            FlintAndSteel,
            Apple,
            Bow,
            Arrow,
            Coal,
            Charcoal,
            Diamond,
            Emerald,
            LapisLazuli,
            Quartz,
            AmethystShard,
            RawIron,
            IronIngot,
            RawCopper,
            CopperIngot,
            RawGold,
            GoldIngot,
            NetheriteIngot,
            NetheriteScrap,
            WoodenSword,
            WoodenShovel,
            WoodenPickaxe,
            WoodenAxe,
            WoodenHoe,
            StoneSword,
            StoneShovel,
            StonePickaxe,
            StoneAxe,
            StoneHoe,
            GoldenSword,
            GoldenShovel,
            GoldenPickaxe,
            GoldenAxe,
            GoldenHoe,
            IronSword,
            IronShovel,
            IronPickaxe,
            IronAxe,
            IronHoe,
            DiamondSword,
            DiamondShovel,
            DiamondPickaxe,
            DiamondAxe,
            DiamondHoe,
            NetheriteSword,
            NetheriteShovel,
            NetheritePickaxe,
            NetheriteAxe,
            NetheriteHoe,
            Stick,
            Bowl,
            MushroomStew,
            String,
            Feather,
            Gunpowder,
            WheatSeeds,
            Wheat,
            Bread,
            LeatherHelmet,
            LeatherChestplate,
            LeatherLeggings,
            LeatherBoots,
            ChainmailHelmet,
            ChainmailChestplate,
            ChainmailLeggings,
            ChainmailBoots,
            IronHelmet,
            IronChestplate,
            IronLeggings,
            IronBoots,
            DiamondHelmet,
            DiamondChestplate,
            DiamondLeggings,
            DiamondBoots,
            GoldenHelmet,
            GoldenChestplate,
            GoldenLeggings,
            GoldenBoots,
            NetheriteHelmet,
            NetheriteChestplate,
            NetheriteLeggings,
            NetheriteBoots,
            Flint,
            Porkchop,
            CookedPorkchop,
            Painting,
            GoldenApple,
            EnchantedGoldenApple,
            OakSign,
            SpruceSign,
            BirchSign,
            JungleSign,
            AcaciaSign,
            DarkOakSign,
            CrimsonSign,
            WarpedSign,
            Bucket,
            WaterBucket,
            LavaBucket,
            PowderSnowBucket,
            Snowball,
            Leather,
            MilkBucket,
            PufferfishBucket,
            SalmonBucket,
            CodBucket,
            TropicalFishBucket,
            AxolotlBucket,
            Brick,
            ClayBall,
            DriedKelpBlock,
            Paper,
            Book,
            SlimeBall,
            Egg,
            Compass,
            Bundle,
            FishingRod,
            Clock,
            Spyglass,
            GlowstoneDust,
            Cod,
            Salmon,
            TropicalFish,
            Pufferfish,
            CookedCod,
            CookedSalmon,
            InkSac,
            GlowInkSac,
            CocoaBeans,
            WhiteDye,
            OrangeDye,
            MagentaDye,
            LightBlueDye,
            YellowDye,
            LimeDye,
            PinkDye,
            GrayDye,
            LightGrayDye,
            CyanDye,
            PurpleDye,
            BlueDye,
            BrownDye,
            GreenDye,
            RedDye,
            BlackDye,
            BoneMeal,
            Bone,
            Sugar,
            Cake,
            WhiteBed,
            OrangeBed,
            MagentaBed,
            LightBlueBed,
            YellowBed,
            LimeBed,
            PinkBed,
            GrayBed,
            LightGrayBed,
            CyanBed,
            PurpleBed,
            BlueBed,
            BrownBed,
            GreenBed,
            RedBed,
            BlackBed,
            Cookie,
            FilledMap,
            Shears,
            MelonSlice,
            DriedKelp,
            PumpkinSeeds,
            MelonSeeds,
            Beef,
            CookedBeef,
            Chicken,
            CookedChicken,
            RottenFlesh,
            EnderPearl,
            BlazeRod,
            GhastTear,
            GoldNugget,
            NetherWart,
            Potion,
            GlassBottle,
            SpiderEye,
            FermentedSpiderEye,
            BlazePowder,
            MagmaCream,
            BrewingStand,
            Cauldron,
            EnderEye,
            GlisteringMelonSlice,
            AxolotlSpawnEgg,
            BatSpawnEgg,
            BeeSpawnEgg,
            BlazeSpawnEgg,
            CatSpawnEgg,
            CaveSpiderSpawnEgg,
            ChickenSpawnEgg,
            CodSpawnEgg,
            CowSpawnEgg,
            CreeperSpawnEgg,
            DolphinSpawnEgg,
            DonkeySpawnEgg,
            DrownedSpawnEgg,
            ElderGuardianSpawnEgg,
            EndermanSpawnEgg,
            EndermiteSpawnEgg,
            EvokerSpawnEgg,
            FoxSpawnEgg,
            GhastSpawnEgg,
            GlowSquidSpawnEgg,
            GoatSpawnEgg,
            GuardianSpawnEgg,
            HoglinSpawnEgg,
            HorseSpawnEgg,
            HuskSpawnEgg,
            LlamaSpawnEgg,
            MagmaCubeSpawnEgg,
            MooshroomSpawnEgg,
            MuleSpawnEgg,
            OcelotSpawnEgg,
            PandaSpawnEgg,
            ParrotSpawnEgg,
            PhantomSpawnEgg,
            PigSpawnEgg,
            PiglinSpawnEgg,
            PiglinBruteSpawnEgg,
            PillagerSpawnEgg,
            PolarBearSpawnEgg,
            PufferfishSpawnEgg,
            RabbitSpawnEgg,
            RavagerSpawnEgg,
            SalmonSpawnEgg,
            SheepSpawnEgg,
            ShulkerSpawnEgg,
            SilverfishSpawnEgg,
            SkeletonSpawnEgg,
            SkeletonHorseSpawnEgg,
            SlimeSpawnEgg,
            SpiderSpawnEgg,
            SquidSpawnEgg,
            StraySpawnEgg,
            StriderSpawnEgg,
            TraderLlamaSpawnEgg,
            TropicalFishSpawnEgg,
            TurtleSpawnEgg,
            VexSpawnEgg,
            VillagerSpawnEgg,
            VindicatorSpawnEgg,
            WanderingTraderSpawnEgg,
            WitchSpawnEgg,
            WitherSkeletonSpawnEgg,
            WolfSpawnEgg,
            ZoglinSpawnEgg,
            ZombieSpawnEgg,
            ZombieHorseSpawnEgg,
            ZombieVillagerSpawnEgg,
            ZombifiedPiglinSpawnEgg,
            ExperienceBottle,
            FireCharge,
            WritableBook,
            WrittenBook,
            ItemFrame,
            GlowItemFrame,
            FlowerPot,
            Carrot,
            Potato,
            BakedPotato,
            PoisonousPotato,
            Map,
            GoldenCarrot,
            SkeletonSkull,
            WitherSkeletonSkull,
            PlayerHead,
            ZombieHead,
            CreeperHead,
            DragonHead,
            NetherStar,
            PumpkinPie,
            FireworkRocket,
            FireworkStar,
            EnchantedBook,
            NetherBrick,
            PrismarineShard,
            PrismarineCrystals,
            Rabbit,
            CookedRabbit,
            RabbitStew,
            RabbitFoot,
            RabbitHide,
            ArmorStand,
            IronHorseArmor,
            GoldenHorseArmor,
            DiamondHorseArmor,
            LeatherHorseArmor,
            Lead,
            NameTag,
            CommandBlockMinecart,
            Mutton,
            CookedMutton,
            WhiteBanner,
            OrangeBanner,
            MagentaBanner,
            LightBlueBanner,
            YellowBanner,
            LimeBanner,
            PinkBanner,
            GrayBanner,
            LightGrayBanner,
            CyanBanner,
            PurpleBanner,
            BlueBanner,
            BrownBanner,
            GreenBanner,
            RedBanner,
            BlackBanner,
            EndCrystal,
            ChorusFruit,
            PoppedChorusFruit,
            Beetroot,
            BeetrootSeeds,
            BeetrootSoup,
            DragonBreath,
            SplashPotion,
            SpectralArrow,
            TippedArrow,
            LingeringPotion,
            Shield,
            TotemOfUndying,
            ShulkerShell,
            IronNugget,
            KnowledgeBook,
            DebugStick,
            MusicDisc13,
            MusicDiscCat,
            MusicDiscBlocks,
            MusicDiscChirp,
            MusicDiscFar,
            MusicDiscMall,
            MusicDiscMellohi,
            MusicDiscStal,
            MusicDiscStrad,
            MusicDiscWard,
            MusicDisc11,
            MusicDiscWait,
            MusicDiscOtherside,
            MusicDiscPigstep,
            Trident,
            PhantomMembrane,
            NautilusShell,
            HeartOfTheSea,
            Crossbow,
            SuspiciousStew,
            Loom,
            FlowerBannerPattern,
            CreeperBannerPattern,
            SkullBannerPattern,
            MojangBannerPattern,
            GlobeBannerPattern,
            PiglinBannerPattern,
            Composter,
            Barrel,
            Smoker,
            BlastFurnace,
            CartographyTable,
            FletchingTable,
            Grindstone,
            SmithingTable,
            Stonecutter,
            Bell,
            Lantern,
            SoulLantern,
            SweetBerries,
            GlowBerries,
            Campfire,
            SoulCampfire,
            Shroomlight,
            Honeycomb,
            BeeNest,
            Beehive,
            HoneyBottle,
            HoneycombBlock,
            Lodestone,
            CryingObsidian,
            Blackstone,
            BlackstoneSlab,
            BlackstoneStairs,
            GildedBlackstone,
            PolishedBlackstone,
            PolishedBlackstoneSlab,
            PolishedBlackstoneStairs,
            ChiseledPolishedBlackstone,
            PolishedBlackstoneBricks,
            PolishedBlackstoneBrickSlab,
            PolishedBlackstoneBrickStairs,
            CrackedPolishedBlackstoneBricks,
            RespawnAnchor,
            Candle,
            WhiteCandle,
            OrangeCandle,
            MagentaCandle,
            LightBlueCandle,
            YellowCandle,
            LimeCandle,
            PinkCandle,
            GrayCandle,
            LightGrayCandle,
            CyanCandle,
            PurpleCandle,
            BlueCandle,
            BrownCandle,
            GreenCandle,
            RedCandle,
            BlackCandle,
            SmallAmethystBud,
            MediumAmethystBud,
            LargeAmethystBud,
            AmethystCluster,
            PointedDripstone,
        ]
    }
}
impl ItemKind {
    #[doc = "Returns the `id` property of this `ItemKind`."]
    #[inline]
    pub const fn id(&self) -> u32 {
        match self {
            ItemKind::Stone => 1u32,
            ItemKind::Granite => 2u32,
            ItemKind::PolishedGranite => 3u32,
            ItemKind::Diorite => 4u32,
            ItemKind::PolishedDiorite => 5u32,
            ItemKind::Andesite => 6u32,
            ItemKind::PolishedAndesite => 7u32,
            ItemKind::Deepslate => 8u32,
            ItemKind::CobbledDeepslate => 9u32,
            ItemKind::PolishedDeepslate => 10u32,
            ItemKind::Calcite => 11u32,
            ItemKind::Tuff => 12u32,
            ItemKind::DripstoneBlock => 13u32,
            ItemKind::GrassBlock => 14u32,
            ItemKind::Dirt => 15u32,
            ItemKind::CoarseDirt => 16u32,
            ItemKind::Podzol => 17u32,
            ItemKind::RootedDirt => 18u32,
            ItemKind::CrimsonNylium => 19u32,
            ItemKind::WarpedNylium => 20u32,
            ItemKind::Cobblestone => 21u32,
            ItemKind::OakPlanks => 22u32,
            ItemKind::SprucePlanks => 23u32,
            ItemKind::BirchPlanks => 24u32,
            ItemKind::JunglePlanks => 25u32,
            ItemKind::AcaciaPlanks => 26u32,
            ItemKind::DarkOakPlanks => 27u32,
            ItemKind::CrimsonPlanks => 28u32,
            ItemKind::WarpedPlanks => 29u32,
            ItemKind::OakSapling => 30u32,
            ItemKind::SpruceSapling => 31u32,
            ItemKind::BirchSapling => 32u32,
            ItemKind::JungleSapling => 33u32,
            ItemKind::AcaciaSapling => 34u32,
            ItemKind::DarkOakSapling => 35u32,
            ItemKind::Bedrock => 36u32,
            ItemKind::Sand => 37u32,
            ItemKind::RedSand => 38u32,
            ItemKind::Gravel => 39u32,
            ItemKind::CoalOre => 40u32,
            ItemKind::DeepslateCoalOre => 41u32,
            ItemKind::IronOre => 42u32,
            ItemKind::DeepslateIronOre => 43u32,
            ItemKind::CopperOre => 44u32,
            ItemKind::DeepslateCopperOre => 45u32,
            ItemKind::GoldOre => 46u32,
            ItemKind::DeepslateGoldOre => 47u32,
            ItemKind::RedstoneOre => 48u32,
            ItemKind::DeepslateRedstoneOre => 49u32,
            ItemKind::EmeraldOre => 50u32,
            ItemKind::DeepslateEmeraldOre => 51u32,
            ItemKind::LapisOre => 52u32,
            ItemKind::DeepslateLapisOre => 53u32,
            ItemKind::DiamondOre => 54u32,
            ItemKind::DeepslateDiamondOre => 55u32,
            ItemKind::NetherGoldOre => 56u32,
            ItemKind::NetherQuartzOre => 57u32,
            ItemKind::AncientDebris => 58u32,
            ItemKind::CoalBlock => 59u32,
            ItemKind::RawIronBlock => 60u32,
            ItemKind::RawCopperBlock => 61u32,
            ItemKind::RawGoldBlock => 62u32,
            ItemKind::AmethystBlock => 63u32,
            ItemKind::BuddingAmethyst => 64u32,
            ItemKind::IronBlock => 65u32,
            ItemKind::CopperBlock => 66u32,
            ItemKind::GoldBlock => 67u32,
            ItemKind::DiamondBlock => 68u32,
            ItemKind::NetheriteBlock => 69u32,
            ItemKind::ExposedCopper => 70u32,
            ItemKind::WeatheredCopper => 71u32,
            ItemKind::OxidizedCopper => 72u32,
            ItemKind::CutCopper => 73u32,
            ItemKind::ExposedCutCopper => 74u32,
            ItemKind::WeatheredCutCopper => 75u32,
            ItemKind::OxidizedCutCopper => 76u32,
            ItemKind::CutCopperStairs => 77u32,
            ItemKind::ExposedCutCopperStairs => 78u32,
            ItemKind::WeatheredCutCopperStairs => 79u32,
            ItemKind::OxidizedCutCopperStairs => 80u32,
            ItemKind::CutCopperSlab => 81u32,
            ItemKind::ExposedCutCopperSlab => 82u32,
            ItemKind::WeatheredCutCopperSlab => 83u32,
            ItemKind::OxidizedCutCopperSlab => 84u32,
            ItemKind::WaxedCopperBlock => 85u32,
            ItemKind::WaxedExposedCopper => 86u32,
            ItemKind::WaxedWeatheredCopper => 87u32,
            ItemKind::WaxedOxidizedCopper => 88u32,
            ItemKind::WaxedCutCopper => 89u32,
            ItemKind::WaxedExposedCutCopper => 90u32,
            ItemKind::WaxedWeatheredCutCopper => 91u32,
            ItemKind::WaxedOxidizedCutCopper => 92u32,
            ItemKind::WaxedCutCopperStairs => 93u32,
            ItemKind::WaxedExposedCutCopperStairs => 94u32,
            ItemKind::WaxedWeatheredCutCopperStairs => 95u32,
            ItemKind::WaxedOxidizedCutCopperStairs => 96u32,
            ItemKind::WaxedCutCopperSlab => 97u32,
            ItemKind::WaxedExposedCutCopperSlab => 98u32,
            ItemKind::WaxedWeatheredCutCopperSlab => 99u32,
            ItemKind::WaxedOxidizedCutCopperSlab => 100u32,
            ItemKind::OakLog => 101u32,
            ItemKind::SpruceLog => 102u32,
            ItemKind::BirchLog => 103u32,
            ItemKind::JungleLog => 104u32,
            ItemKind::AcaciaLog => 105u32,
            ItemKind::DarkOakLog => 106u32,
            ItemKind::CrimsonStem => 107u32,
            ItemKind::WarpedStem => 108u32,
            ItemKind::StrippedOakLog => 109u32,
            ItemKind::StrippedSpruceLog => 110u32,
            ItemKind::StrippedBirchLog => 111u32,
            ItemKind::StrippedJungleLog => 112u32,
            ItemKind::StrippedAcaciaLog => 113u32,
            ItemKind::StrippedDarkOakLog => 114u32,
            ItemKind::StrippedCrimsonStem => 115u32,
            ItemKind::StrippedWarpedStem => 116u32,
            ItemKind::StrippedOakWood => 117u32,
            ItemKind::StrippedSpruceWood => 118u32,
            ItemKind::StrippedBirchWood => 119u32,
            ItemKind::StrippedJungleWood => 120u32,
            ItemKind::StrippedAcaciaWood => 121u32,
            ItemKind::StrippedDarkOakWood => 122u32,
            ItemKind::StrippedCrimsonHyphae => 123u32,
            ItemKind::StrippedWarpedHyphae => 124u32,
            ItemKind::OakWood => 125u32,
            ItemKind::SpruceWood => 126u32,
            ItemKind::BirchWood => 127u32,
            ItemKind::JungleWood => 128u32,
            ItemKind::AcaciaWood => 129u32,
            ItemKind::DarkOakWood => 130u32,
            ItemKind::CrimsonHyphae => 131u32,
            ItemKind::WarpedHyphae => 132u32,
            ItemKind::OakLeaves => 133u32,
            ItemKind::SpruceLeaves => 134u32,
            ItemKind::BirchLeaves => 135u32,
            ItemKind::JungleLeaves => 136u32,
            ItemKind::AcaciaLeaves => 137u32,
            ItemKind::DarkOakLeaves => 138u32,
            ItemKind::AzaleaLeaves => 139u32,
            ItemKind::FloweringAzaleaLeaves => 140u32,
            ItemKind::Sponge => 141u32,
            ItemKind::WetSponge => 142u32,
            ItemKind::Glass => 143u32,
            ItemKind::TintedGlass => 144u32,
            ItemKind::LapisBlock => 145u32,
            ItemKind::Sandstone => 146u32,
            ItemKind::ChiseledSandstone => 147u32,
            ItemKind::CutSandstone => 148u32,
            ItemKind::Cobweb => 149u32,
            ItemKind::Grass => 150u32,
            ItemKind::Fern => 151u32,
            ItemKind::Azalea => 152u32,
            ItemKind::FloweringAzalea => 153u32,
            ItemKind::DeadBush => 154u32,
            ItemKind::Seagrass => 155u32,
            ItemKind::SeaPickle => 156u32,
            ItemKind::WhiteWool => 157u32,
            ItemKind::OrangeWool => 158u32,
            ItemKind::MagentaWool => 159u32,
            ItemKind::LightBlueWool => 160u32,
            ItemKind::YellowWool => 161u32,
            ItemKind::LimeWool => 162u32,
            ItemKind::PinkWool => 163u32,
            ItemKind::GrayWool => 164u32,
            ItemKind::LightGrayWool => 165u32,
            ItemKind::CyanWool => 166u32,
            ItemKind::PurpleWool => 167u32,
            ItemKind::BlueWool => 168u32,
            ItemKind::BrownWool => 169u32,
            ItemKind::GreenWool => 170u32,
            ItemKind::RedWool => 171u32,
            ItemKind::BlackWool => 172u32,
            ItemKind::Dandelion => 173u32,
            ItemKind::Poppy => 174u32,
            ItemKind::BlueOrchid => 175u32,
            ItemKind::Allium => 176u32,
            ItemKind::AzureBluet => 177u32,
            ItemKind::RedTulip => 178u32,
            ItemKind::OrangeTulip => 179u32,
            ItemKind::WhiteTulip => 180u32,
            ItemKind::PinkTulip => 181u32,
            ItemKind::OxeyeDaisy => 182u32,
            ItemKind::Cornflower => 183u32,
            ItemKind::LilyOfTheValley => 184u32,
            ItemKind::WitherRose => 185u32,
            ItemKind::SporeBlossom => 186u32,
            ItemKind::BrownMushroom => 187u32,
            ItemKind::RedMushroom => 188u32,
            ItemKind::CrimsonFungus => 189u32,
            ItemKind::WarpedFungus => 190u32,
            ItemKind::CrimsonRoots => 191u32,
            ItemKind::WarpedRoots => 192u32,
            ItemKind::NetherSprouts => 193u32,
            ItemKind::WeepingVines => 194u32,
            ItemKind::TwistingVines => 195u32,
            ItemKind::SugarCane => 196u32,
            ItemKind::Kelp => 197u32,
            ItemKind::MossCarpet => 198u32,
            ItemKind::MossBlock => 199u32,
            ItemKind::HangingRoots => 200u32,
            ItemKind::BigDripleaf => 201u32,
            ItemKind::SmallDripleaf => 202u32,
            ItemKind::Bamboo => 203u32,
            ItemKind::OakSlab => 204u32,
            ItemKind::SpruceSlab => 205u32,
            ItemKind::BirchSlab => 206u32,
            ItemKind::JungleSlab => 207u32,
            ItemKind::AcaciaSlab => 208u32,
            ItemKind::DarkOakSlab => 209u32,
            ItemKind::CrimsonSlab => 210u32,
            ItemKind::WarpedSlab => 211u32,
            ItemKind::StoneSlab => 212u32,
            ItemKind::SmoothStoneSlab => 213u32,
            ItemKind::SandstoneSlab => 214u32,
            ItemKind::CutSandstoneSlab => 215u32,
            ItemKind::PetrifiedOakSlab => 216u32,
            ItemKind::CobblestoneSlab => 217u32,
            ItemKind::BrickSlab => 218u32,
            ItemKind::StoneBrickSlab => 219u32,
            ItemKind::NetherBrickSlab => 220u32,
            ItemKind::QuartzSlab => 221u32,
            ItemKind::RedSandstoneSlab => 222u32,
            ItemKind::CutRedSandstoneSlab => 223u32,
            ItemKind::PurpurSlab => 224u32,
            ItemKind::PrismarineSlab => 225u32,
            ItemKind::PrismarineBrickSlab => 226u32,
            ItemKind::DarkPrismarineSlab => 227u32,
            ItemKind::SmoothQuartz => 228u32,
            ItemKind::SmoothRedSandstone => 229u32,
            ItemKind::SmoothSandstone => 230u32,
            ItemKind::SmoothStone => 231u32,
            ItemKind::Bricks => 232u32,
            ItemKind::Bookshelf => 233u32,
            ItemKind::MossyCobblestone => 234u32,
            ItemKind::Obsidian => 235u32,
            ItemKind::Torch => 236u32,
            ItemKind::EndRod => 237u32,
            ItemKind::ChorusPlant => 238u32,
            ItemKind::ChorusFlower => 239u32,
            ItemKind::PurpurBlock => 240u32,
            ItemKind::PurpurPillar => 241u32,
            ItemKind::PurpurStairs => 242u32,
            ItemKind::Spawner => 243u32,
            ItemKind::OakStairs => 244u32,
            ItemKind::Chest => 245u32,
            ItemKind::CraftingTable => 246u32,
            ItemKind::Farmland => 247u32,
            ItemKind::Furnace => 248u32,
            ItemKind::Ladder => 249u32,
            ItemKind::CobblestoneStairs => 250u32,
            ItemKind::Snow => 251u32,
            ItemKind::Ice => 252u32,
            ItemKind::SnowBlock => 253u32,
            ItemKind::Cactus => 254u32,
            ItemKind::Clay => 255u32,
            ItemKind::Jukebox => 256u32,
            ItemKind::OakFence => 257u32,
            ItemKind::SpruceFence => 258u32,
            ItemKind::BirchFence => 259u32,
            ItemKind::JungleFence => 260u32,
            ItemKind::AcaciaFence => 261u32,
            ItemKind::DarkOakFence => 262u32,
            ItemKind::CrimsonFence => 263u32,
            ItemKind::WarpedFence => 264u32,
            ItemKind::Pumpkin => 265u32,
            ItemKind::CarvedPumpkin => 266u32,
            ItemKind::JackOLantern => 267u32,
            ItemKind::Netherrack => 268u32,
            ItemKind::SoulSand => 269u32,
            ItemKind::SoulSoil => 270u32,
            ItemKind::Basalt => 271u32,
            ItemKind::PolishedBasalt => 272u32,
            ItemKind::SmoothBasalt => 273u32,
            ItemKind::SoulTorch => 274u32,
            ItemKind::Glowstone => 275u32,
            ItemKind::InfestedStone => 276u32,
            ItemKind::InfestedCobblestone => 277u32,
            ItemKind::InfestedStoneBricks => 278u32,
            ItemKind::InfestedMossyStoneBricks => 279u32,
            ItemKind::InfestedCrackedStoneBricks => 280u32,
            ItemKind::InfestedChiseledStoneBricks => 281u32,
            ItemKind::InfestedDeepslate => 282u32,
            ItemKind::StoneBricks => 283u32,
            ItemKind::MossyStoneBricks => 284u32,
            ItemKind::CrackedStoneBricks => 285u32,
            ItemKind::ChiseledStoneBricks => 286u32,
            ItemKind::DeepslateBricks => 287u32,
            ItemKind::CrackedDeepslateBricks => 288u32,
            ItemKind::DeepslateTiles => 289u32,
            ItemKind::CrackedDeepslateTiles => 290u32,
            ItemKind::ChiseledDeepslate => 291u32,
            ItemKind::BrownMushroomBlock => 292u32,
            ItemKind::RedMushroomBlock => 293u32,
            ItemKind::MushroomStem => 294u32,
            ItemKind::IronBars => 295u32,
            ItemKind::Chain => 296u32,
            ItemKind::GlassPane => 297u32,
            ItemKind::Melon => 298u32,
            ItemKind::Vine => 299u32,
            ItemKind::GlowLichen => 300u32,
            ItemKind::BrickStairs => 301u32,
            ItemKind::StoneBrickStairs => 302u32,
            ItemKind::Mycelium => 303u32,
            ItemKind::LilyPad => 304u32,
            ItemKind::NetherBricks => 305u32,
            ItemKind::CrackedNetherBricks => 306u32,
            ItemKind::ChiseledNetherBricks => 307u32,
            ItemKind::NetherBrickFence => 308u32,
            ItemKind::NetherBrickStairs => 309u32,
            ItemKind::EnchantingTable => 310u32,
            ItemKind::EndPortalFrame => 311u32,
            ItemKind::EndStone => 312u32,
            ItemKind::EndStoneBricks => 313u32,
            ItemKind::DragonEgg => 314u32,
            ItemKind::SandstoneStairs => 315u32,
            ItemKind::EnderChest => 316u32,
            ItemKind::EmeraldBlock => 317u32,
            ItemKind::SpruceStairs => 318u32,
            ItemKind::BirchStairs => 319u32,
            ItemKind::JungleStairs => 320u32,
            ItemKind::CrimsonStairs => 321u32,
            ItemKind::WarpedStairs => 322u32,
            ItemKind::CommandBlock => 323u32,
            ItemKind::Beacon => 324u32,
            ItemKind::CobblestoneWall => 325u32,
            ItemKind::MossyCobblestoneWall => 326u32,
            ItemKind::BrickWall => 327u32,
            ItemKind::PrismarineWall => 328u32,
            ItemKind::RedSandstoneWall => 329u32,
            ItemKind::MossyStoneBrickWall => 330u32,
            ItemKind::GraniteWall => 331u32,
            ItemKind::StoneBrickWall => 332u32,
            ItemKind::NetherBrickWall => 333u32,
            ItemKind::AndesiteWall => 334u32,
            ItemKind::RedNetherBrickWall => 335u32,
            ItemKind::SandstoneWall => 336u32,
            ItemKind::EndStoneBrickWall => 337u32,
            ItemKind::DioriteWall => 338u32,
            ItemKind::BlackstoneWall => 339u32,
            ItemKind::PolishedBlackstoneWall => 340u32,
            ItemKind::PolishedBlackstoneBrickWall => 341u32,
            ItemKind::CobbledDeepslateWall => 342u32,
            ItemKind::PolishedDeepslateWall => 343u32,
            ItemKind::DeepslateBrickWall => 344u32,
            ItemKind::DeepslateTileWall => 345u32,
            ItemKind::Anvil => 346u32,
            ItemKind::ChippedAnvil => 347u32,
            ItemKind::DamagedAnvil => 348u32,
            ItemKind::ChiseledQuartzBlock => 349u32,
            ItemKind::QuartzBlock => 350u32,
            ItemKind::QuartzBricks => 351u32,
            ItemKind::QuartzPillar => 352u32,
            ItemKind::QuartzStairs => 353u32,
            ItemKind::WhiteTerracotta => 354u32,
            ItemKind::OrangeTerracotta => 355u32,
            ItemKind::MagentaTerracotta => 356u32,
            ItemKind::LightBlueTerracotta => 357u32,
            ItemKind::YellowTerracotta => 358u32,
            ItemKind::LimeTerracotta => 359u32,
            ItemKind::PinkTerracotta => 360u32,
            ItemKind::GrayTerracotta => 361u32,
            ItemKind::LightGrayTerracotta => 362u32,
            ItemKind::CyanTerracotta => 363u32,
            ItemKind::PurpleTerracotta => 364u32,
            ItemKind::BlueTerracotta => 365u32,
            ItemKind::BrownTerracotta => 366u32,
            ItemKind::GreenTerracotta => 367u32,
            ItemKind::RedTerracotta => 368u32,
            ItemKind::BlackTerracotta => 369u32,
            ItemKind::Barrier => 370u32,
            ItemKind::Light => 371u32,
            ItemKind::HayBlock => 372u32,
            ItemKind::WhiteCarpet => 373u32,
            ItemKind::OrangeCarpet => 374u32,
            ItemKind::MagentaCarpet => 375u32,
            ItemKind::LightBlueCarpet => 376u32,
            ItemKind::YellowCarpet => 377u32,
            ItemKind::LimeCarpet => 378u32,
            ItemKind::PinkCarpet => 379u32,
            ItemKind::GrayCarpet => 380u32,
            ItemKind::LightGrayCarpet => 381u32,
            ItemKind::CyanCarpet => 382u32,
            ItemKind::PurpleCarpet => 383u32,
            ItemKind::BlueCarpet => 384u32,
            ItemKind::BrownCarpet => 385u32,
            ItemKind::GreenCarpet => 386u32,
            ItemKind::RedCarpet => 387u32,
            ItemKind::BlackCarpet => 388u32,
            ItemKind::Terracotta => 389u32,
            ItemKind::PackedIce => 390u32,
            ItemKind::AcaciaStairs => 391u32,
            ItemKind::DarkOakStairs => 392u32,
            ItemKind::DirtPath => 393u32,
            ItemKind::Sunflower => 394u32,
            ItemKind::Lilac => 395u32,
            ItemKind::RoseBush => 396u32,
            ItemKind::Peony => 397u32,
            ItemKind::TallGrass => 398u32,
            ItemKind::LargeFern => 399u32,
            ItemKind::WhiteStainedGlass => 400u32,
            ItemKind::OrangeStainedGlass => 401u32,
            ItemKind::MagentaStainedGlass => 402u32,
            ItemKind::LightBlueStainedGlass => 403u32,
            ItemKind::YellowStainedGlass => 404u32,
            ItemKind::LimeStainedGlass => 405u32,
            ItemKind::PinkStainedGlass => 406u32,
            ItemKind::GrayStainedGlass => 407u32,
            ItemKind::LightGrayStainedGlass => 408u32,
            ItemKind::CyanStainedGlass => 409u32,
            ItemKind::PurpleStainedGlass => 410u32,
            ItemKind::BlueStainedGlass => 411u32,
            ItemKind::BrownStainedGlass => 412u32,
            ItemKind::GreenStainedGlass => 413u32,
            ItemKind::RedStainedGlass => 414u32,
            ItemKind::BlackStainedGlass => 415u32,
            ItemKind::WhiteStainedGlassPane => 416u32,
            ItemKind::OrangeStainedGlassPane => 417u32,
            ItemKind::MagentaStainedGlassPane => 418u32,
            ItemKind::LightBlueStainedGlassPane => 419u32,
            ItemKind::YellowStainedGlassPane => 420u32,
            ItemKind::LimeStainedGlassPane => 421u32,
            ItemKind::PinkStainedGlassPane => 422u32,
            ItemKind::GrayStainedGlassPane => 423u32,
            ItemKind::LightGrayStainedGlassPane => 424u32,
            ItemKind::CyanStainedGlassPane => 425u32,
            ItemKind::PurpleStainedGlassPane => 426u32,
            ItemKind::BlueStainedGlassPane => 427u32,
            ItemKind::BrownStainedGlassPane => 428u32,
            ItemKind::GreenStainedGlassPane => 429u32,
            ItemKind::RedStainedGlassPane => 430u32,
            ItemKind::BlackStainedGlassPane => 431u32,
            ItemKind::Prismarine => 432u32,
            ItemKind::PrismarineBricks => 433u32,
            ItemKind::DarkPrismarine => 434u32,
            ItemKind::PrismarineStairs => 435u32,
            ItemKind::PrismarineBrickStairs => 436u32,
            ItemKind::DarkPrismarineStairs => 437u32,
            ItemKind::SeaLantern => 438u32,
            ItemKind::RedSandstone => 439u32,
            ItemKind::ChiseledRedSandstone => 440u32,
            ItemKind::CutRedSandstone => 441u32,
            ItemKind::RedSandstoneStairs => 442u32,
            ItemKind::RepeatingCommandBlock => 443u32,
            ItemKind::ChainCommandBlock => 444u32,
            ItemKind::MagmaBlock => 445u32,
            ItemKind::NetherWartBlock => 446u32,
            ItemKind::WarpedWartBlock => 447u32,
            ItemKind::RedNetherBricks => 448u32,
            ItemKind::BoneBlock => 449u32,
            ItemKind::StructureVoid => 450u32,
            ItemKind::ShulkerBox => 451u32,
            ItemKind::WhiteShulkerBox => 452u32,
            ItemKind::OrangeShulkerBox => 453u32,
            ItemKind::MagentaShulkerBox => 454u32,
            ItemKind::LightBlueShulkerBox => 455u32,
            ItemKind::YellowShulkerBox => 456u32,
            ItemKind::LimeShulkerBox => 457u32,
            ItemKind::PinkShulkerBox => 458u32,
            ItemKind::GrayShulkerBox => 459u32,
            ItemKind::LightGrayShulkerBox => 460u32,
            ItemKind::CyanShulkerBox => 461u32,
            ItemKind::PurpleShulkerBox => 462u32,
            ItemKind::BlueShulkerBox => 463u32,
            ItemKind::BrownShulkerBox => 464u32,
            ItemKind::GreenShulkerBox => 465u32,
            ItemKind::RedShulkerBox => 466u32,
            ItemKind::BlackShulkerBox => 467u32,
            ItemKind::WhiteGlazedTerracotta => 468u32,
            ItemKind::OrangeGlazedTerracotta => 469u32,
            ItemKind::MagentaGlazedTerracotta => 470u32,
            ItemKind::LightBlueGlazedTerracotta => 471u32,
            ItemKind::YellowGlazedTerracotta => 472u32,
            ItemKind::LimeGlazedTerracotta => 473u32,
            ItemKind::PinkGlazedTerracotta => 474u32,
            ItemKind::GrayGlazedTerracotta => 475u32,
            ItemKind::LightGrayGlazedTerracotta => 476u32,
            ItemKind::CyanGlazedTerracotta => 477u32,
            ItemKind::PurpleGlazedTerracotta => 478u32,
            ItemKind::BlueGlazedTerracotta => 479u32,
            ItemKind::BrownGlazedTerracotta => 480u32,
            ItemKind::GreenGlazedTerracotta => 481u32,
            ItemKind::RedGlazedTerracotta => 482u32,
            ItemKind::BlackGlazedTerracotta => 483u32,
            ItemKind::WhiteConcrete => 484u32,
            ItemKind::OrangeConcrete => 485u32,
            ItemKind::MagentaConcrete => 486u32,
            ItemKind::LightBlueConcrete => 487u32,
            ItemKind::YellowConcrete => 488u32,
            ItemKind::LimeConcrete => 489u32,
            ItemKind::PinkConcrete => 490u32,
            ItemKind::GrayConcrete => 491u32,
            ItemKind::LightGrayConcrete => 492u32,
            ItemKind::CyanConcrete => 493u32,
            ItemKind::PurpleConcrete => 494u32,
            ItemKind::BlueConcrete => 495u32,
            ItemKind::BrownConcrete => 496u32,
            ItemKind::GreenConcrete => 497u32,
            ItemKind::RedConcrete => 498u32,
            ItemKind::BlackConcrete => 499u32,
            ItemKind::WhiteConcretePowder => 500u32,
            ItemKind::OrangeConcretePowder => 501u32,
            ItemKind::MagentaConcretePowder => 502u32,
            ItemKind::LightBlueConcretePowder => 503u32,
            ItemKind::YellowConcretePowder => 504u32,
            ItemKind::LimeConcretePowder => 505u32,
            ItemKind::PinkConcretePowder => 506u32,
            ItemKind::GrayConcretePowder => 507u32,
            ItemKind::LightGrayConcretePowder => 508u32,
            ItemKind::CyanConcretePowder => 509u32,
            ItemKind::PurpleConcretePowder => 510u32,
            ItemKind::BlueConcretePowder => 511u32,
            ItemKind::BrownConcretePowder => 512u32,
            ItemKind::GreenConcretePowder => 513u32,
            ItemKind::RedConcretePowder => 514u32,
            ItemKind::BlackConcretePowder => 515u32,
            ItemKind::TurtleEgg => 516u32,
            ItemKind::DeadTubeCoralBlock => 517u32,
            ItemKind::DeadBrainCoralBlock => 518u32,
            ItemKind::DeadBubbleCoralBlock => 519u32,
            ItemKind::DeadFireCoralBlock => 520u32,
            ItemKind::DeadHornCoralBlock => 521u32,
            ItemKind::TubeCoralBlock => 522u32,
            ItemKind::BrainCoralBlock => 523u32,
            ItemKind::BubbleCoralBlock => 524u32,
            ItemKind::FireCoralBlock => 525u32,
            ItemKind::HornCoralBlock => 526u32,
            ItemKind::TubeCoral => 527u32,
            ItemKind::BrainCoral => 528u32,
            ItemKind::BubbleCoral => 529u32,
            ItemKind::FireCoral => 530u32,
            ItemKind::HornCoral => 531u32,
            ItemKind::DeadBrainCoral => 532u32,
            ItemKind::DeadBubbleCoral => 533u32,
            ItemKind::DeadFireCoral => 534u32,
            ItemKind::DeadHornCoral => 535u32,
            ItemKind::DeadTubeCoral => 536u32,
            ItemKind::TubeCoralFan => 537u32,
            ItemKind::BrainCoralFan => 538u32,
            ItemKind::BubbleCoralFan => 539u32,
            ItemKind::FireCoralFan => 540u32,
            ItemKind::HornCoralFan => 541u32,
            ItemKind::DeadTubeCoralFan => 542u32,
            ItemKind::DeadBrainCoralFan => 543u32,
            ItemKind::DeadBubbleCoralFan => 544u32,
            ItemKind::DeadFireCoralFan => 545u32,
            ItemKind::DeadHornCoralFan => 546u32,
            ItemKind::BlueIce => 547u32,
            ItemKind::Conduit => 548u32,
            ItemKind::PolishedGraniteStairs => 549u32,
            ItemKind::SmoothRedSandstoneStairs => 550u32,
            ItemKind::MossyStoneBrickStairs => 551u32,
            ItemKind::PolishedDioriteStairs => 552u32,
            ItemKind::MossyCobblestoneStairs => 553u32,
            ItemKind::EndStoneBrickStairs => 554u32,
            ItemKind::StoneStairs => 555u32,
            ItemKind::SmoothSandstoneStairs => 556u32,
            ItemKind::SmoothQuartzStairs => 557u32,
            ItemKind::GraniteStairs => 558u32,
            ItemKind::AndesiteStairs => 559u32,
            ItemKind::RedNetherBrickStairs => 560u32,
            ItemKind::PolishedAndesiteStairs => 561u32,
            ItemKind::DioriteStairs => 562u32,
            ItemKind::CobbledDeepslateStairs => 563u32,
            ItemKind::PolishedDeepslateStairs => 564u32,
            ItemKind::DeepslateBrickStairs => 565u32,
            ItemKind::DeepslateTileStairs => 566u32,
            ItemKind::PolishedGraniteSlab => 567u32,
            ItemKind::SmoothRedSandstoneSlab => 568u32,
            ItemKind::MossyStoneBrickSlab => 569u32,
            ItemKind::PolishedDioriteSlab => 570u32,
            ItemKind::MossyCobblestoneSlab => 571u32,
            ItemKind::EndStoneBrickSlab => 572u32,
            ItemKind::SmoothSandstoneSlab => 573u32,
            ItemKind::SmoothQuartzSlab => 574u32,
            ItemKind::GraniteSlab => 575u32,
            ItemKind::AndesiteSlab => 576u32,
            ItemKind::RedNetherBrickSlab => 577u32,
            ItemKind::PolishedAndesiteSlab => 578u32,
            ItemKind::DioriteSlab => 579u32,
            ItemKind::CobbledDeepslateSlab => 580u32,
            ItemKind::PolishedDeepslateSlab => 581u32,
            ItemKind::DeepslateBrickSlab => 582u32,
            ItemKind::DeepslateTileSlab => 583u32,
            ItemKind::Scaffolding => 584u32,
            ItemKind::Redstone => 585u32,
            ItemKind::RedstoneTorch => 586u32,
            ItemKind::RedstoneBlock => 587u32,
            ItemKind::Repeater => 588u32,
            ItemKind::Comparator => 589u32,
            ItemKind::Piston => 590u32,
            ItemKind::StickyPiston => 591u32,
            ItemKind::SlimeBlock => 592u32,
            ItemKind::HoneyBlock => 593u32,
            ItemKind::Observer => 594u32,
            ItemKind::Hopper => 595u32,
            ItemKind::Dispenser => 596u32,
            ItemKind::Dropper => 597u32,
            ItemKind::Lectern => 598u32,
            ItemKind::Target => 599u32,
            ItemKind::Lever => 600u32,
            ItemKind::LightningRod => 601u32,
            ItemKind::DaylightDetector => 602u32,
            ItemKind::SculkSensor => 603u32,
            ItemKind::TripwireHook => 604u32,
            ItemKind::TrappedChest => 605u32,
            ItemKind::Tnt => 606u32,
            ItemKind::RedstoneLamp => 607u32,
            ItemKind::NoteBlock => 608u32,
            ItemKind::StoneButton => 609u32,
            ItemKind::PolishedBlackstoneButton => 610u32,
            ItemKind::OakButton => 611u32,
            ItemKind::SpruceButton => 612u32,
            ItemKind::BirchButton => 613u32,
            ItemKind::JungleButton => 614u32,
            ItemKind::AcaciaButton => 615u32,
            ItemKind::DarkOakButton => 616u32,
            ItemKind::CrimsonButton => 617u32,
            ItemKind::WarpedButton => 618u32,
            ItemKind::StonePressurePlate => 619u32,
            ItemKind::PolishedBlackstonePressurePlate => 620u32,
            ItemKind::LightWeightedPressurePlate => 621u32,
            ItemKind::HeavyWeightedPressurePlate => 622u32,
            ItemKind::OakPressurePlate => 623u32,
            ItemKind::SprucePressurePlate => 624u32,
            ItemKind::BirchPressurePlate => 625u32,
            ItemKind::JunglePressurePlate => 626u32,
            ItemKind::AcaciaPressurePlate => 627u32,
            ItemKind::DarkOakPressurePlate => 628u32,
            ItemKind::CrimsonPressurePlate => 629u32,
            ItemKind::WarpedPressurePlate => 630u32,
            ItemKind::IronDoor => 631u32,
            ItemKind::OakDoor => 632u32,
            ItemKind::SpruceDoor => 633u32,
            ItemKind::BirchDoor => 634u32,
            ItemKind::JungleDoor => 635u32,
            ItemKind::AcaciaDoor => 636u32,
            ItemKind::DarkOakDoor => 637u32,
            ItemKind::CrimsonDoor => 638u32,
            ItemKind::WarpedDoor => 639u32,
            ItemKind::IronTrapdoor => 640u32,
            ItemKind::OakTrapdoor => 641u32,
            ItemKind::SpruceTrapdoor => 642u32,
            ItemKind::BirchTrapdoor => 643u32,
            ItemKind::JungleTrapdoor => 644u32,
            ItemKind::AcaciaTrapdoor => 645u32,
            ItemKind::DarkOakTrapdoor => 646u32,
            ItemKind::CrimsonTrapdoor => 647u32,
            ItemKind::WarpedTrapdoor => 648u32,
            ItemKind::OakFenceGate => 649u32,
            ItemKind::SpruceFenceGate => 650u32,
            ItemKind::BirchFenceGate => 651u32,
            ItemKind::JungleFenceGate => 652u32,
            ItemKind::AcaciaFenceGate => 653u32,
            ItemKind::DarkOakFenceGate => 654u32,
            ItemKind::CrimsonFenceGate => 655u32,
            ItemKind::WarpedFenceGate => 656u32,
            ItemKind::PoweredRail => 657u32,
            ItemKind::DetectorRail => 658u32,
            ItemKind::Rail => 659u32,
            ItemKind::ActivatorRail => 660u32,
            ItemKind::Saddle => 661u32,
            ItemKind::Minecart => 662u32,
            ItemKind::ChestMinecart => 663u32,
            ItemKind::FurnaceMinecart => 664u32,
            ItemKind::TntMinecart => 665u32,
            ItemKind::HopperMinecart => 666u32,
            ItemKind::CarrotOnAStick => 667u32,
            ItemKind::WarpedFungusOnAStick => 668u32,
            ItemKind::Elytra => 669u32,
            ItemKind::OakBoat => 670u32,
            ItemKind::SpruceBoat => 671u32,
            ItemKind::BirchBoat => 672u32,
            ItemKind::JungleBoat => 673u32,
            ItemKind::AcaciaBoat => 674u32,
            ItemKind::DarkOakBoat => 675u32,
            ItemKind::StructureBlock => 676u32,
            ItemKind::Jigsaw => 677u32,
            ItemKind::TurtleHelmet => 678u32,
            ItemKind::Scute => 679u32,
            ItemKind::FlintAndSteel => 680u32,
            ItemKind::Apple => 681u32,
            ItemKind::Bow => 682u32,
            ItemKind::Arrow => 683u32,
            ItemKind::Coal => 684u32,
            ItemKind::Charcoal => 685u32,
            ItemKind::Diamond => 686u32,
            ItemKind::Emerald => 687u32,
            ItemKind::LapisLazuli => 688u32,
            ItemKind::Quartz => 689u32,
            ItemKind::AmethystShard => 690u32,
            ItemKind::RawIron => 691u32,
            ItemKind::IronIngot => 692u32,
            ItemKind::RawCopper => 693u32,
            ItemKind::CopperIngot => 694u32,
            ItemKind::RawGold => 695u32,
            ItemKind::GoldIngot => 696u32,
            ItemKind::NetheriteIngot => 697u32,
            ItemKind::NetheriteScrap => 698u32,
            ItemKind::WoodenSword => 699u32,
            ItemKind::WoodenShovel => 700u32,
            ItemKind::WoodenPickaxe => 701u32,
            ItemKind::WoodenAxe => 702u32,
            ItemKind::WoodenHoe => 703u32,
            ItemKind::StoneSword => 704u32,
            ItemKind::StoneShovel => 705u32,
            ItemKind::StonePickaxe => 706u32,
            ItemKind::StoneAxe => 707u32,
            ItemKind::StoneHoe => 708u32,
            ItemKind::GoldenSword => 709u32,
            ItemKind::GoldenShovel => 710u32,
            ItemKind::GoldenPickaxe => 711u32,
            ItemKind::GoldenAxe => 712u32,
            ItemKind::GoldenHoe => 713u32,
            ItemKind::IronSword => 714u32,
            ItemKind::IronShovel => 715u32,
            ItemKind::IronPickaxe => 716u32,
            ItemKind::IronAxe => 717u32,
            ItemKind::IronHoe => 718u32,
            ItemKind::DiamondSword => 719u32,
            ItemKind::DiamondShovel => 720u32,
            ItemKind::DiamondPickaxe => 721u32,
            ItemKind::DiamondAxe => 722u32,
            ItemKind::DiamondHoe => 723u32,
            ItemKind::NetheriteSword => 724u32,
            ItemKind::NetheriteShovel => 725u32,
            ItemKind::NetheritePickaxe => 726u32,
            ItemKind::NetheriteAxe => 727u32,
            ItemKind::NetheriteHoe => 728u32,
            ItemKind::Stick => 729u32,
            ItemKind::Bowl => 730u32,
            ItemKind::MushroomStew => 731u32,
            ItemKind::String => 732u32,
            ItemKind::Feather => 733u32,
            ItemKind::Gunpowder => 734u32,
            ItemKind::WheatSeeds => 735u32,
            ItemKind::Wheat => 736u32,
            ItemKind::Bread => 737u32,
            ItemKind::LeatherHelmet => 738u32,
            ItemKind::LeatherChestplate => 739u32,
            ItemKind::LeatherLeggings => 740u32,
            ItemKind::LeatherBoots => 741u32,
            ItemKind::ChainmailHelmet => 742u32,
            ItemKind::ChainmailChestplate => 743u32,
            ItemKind::ChainmailLeggings => 744u32,
            ItemKind::ChainmailBoots => 745u32,
            ItemKind::IronHelmet => 746u32,
            ItemKind::IronChestplate => 747u32,
            ItemKind::IronLeggings => 748u32,
            ItemKind::IronBoots => 749u32,
            ItemKind::DiamondHelmet => 750u32,
            ItemKind::DiamondChestplate => 751u32,
            ItemKind::DiamondLeggings => 752u32,
            ItemKind::DiamondBoots => 753u32,
            ItemKind::GoldenHelmet => 754u32,
            ItemKind::GoldenChestplate => 755u32,
            ItemKind::GoldenLeggings => 756u32,
            ItemKind::GoldenBoots => 757u32,
            ItemKind::NetheriteHelmet => 758u32,
            ItemKind::NetheriteChestplate => 759u32,
            ItemKind::NetheriteLeggings => 760u32,
            ItemKind::NetheriteBoots => 761u32,
            ItemKind::Flint => 762u32,
            ItemKind::Porkchop => 763u32,
            ItemKind::CookedPorkchop => 764u32,
            ItemKind::Painting => 765u32,
            ItemKind::GoldenApple => 766u32,
            ItemKind::EnchantedGoldenApple => 767u32,
            ItemKind::OakSign => 768u32,
            ItemKind::SpruceSign => 769u32,
            ItemKind::BirchSign => 770u32,
            ItemKind::JungleSign => 771u32,
            ItemKind::AcaciaSign => 772u32,
            ItemKind::DarkOakSign => 773u32,
            ItemKind::CrimsonSign => 774u32,
            ItemKind::WarpedSign => 775u32,
            ItemKind::Bucket => 776u32,
            ItemKind::WaterBucket => 777u32,
            ItemKind::LavaBucket => 778u32,
            ItemKind::PowderSnowBucket => 779u32,
            ItemKind::Snowball => 780u32,
            ItemKind::Leather => 781u32,
            ItemKind::MilkBucket => 782u32,
            ItemKind::PufferfishBucket => 783u32,
            ItemKind::SalmonBucket => 784u32,
            ItemKind::CodBucket => 785u32,
            ItemKind::TropicalFishBucket => 786u32,
            ItemKind::AxolotlBucket => 787u32,
            ItemKind::Brick => 788u32,
            ItemKind::ClayBall => 789u32,
            ItemKind::DriedKelpBlock => 790u32,
            ItemKind::Paper => 791u32,
            ItemKind::Book => 792u32,
            ItemKind::SlimeBall => 793u32,
            ItemKind::Egg => 794u32,
            ItemKind::Compass => 795u32,
            ItemKind::Bundle => 796u32,
            ItemKind::FishingRod => 797u32,
            ItemKind::Clock => 798u32,
            ItemKind::Spyglass => 799u32,
            ItemKind::GlowstoneDust => 800u32,
            ItemKind::Cod => 801u32,
            ItemKind::Salmon => 802u32,
            ItemKind::TropicalFish => 803u32,
            ItemKind::Pufferfish => 804u32,
            ItemKind::CookedCod => 805u32,
            ItemKind::CookedSalmon => 806u32,
            ItemKind::InkSac => 807u32,
            ItemKind::GlowInkSac => 808u32,
            ItemKind::CocoaBeans => 809u32,
            ItemKind::WhiteDye => 810u32,
            ItemKind::OrangeDye => 811u32,
            ItemKind::MagentaDye => 812u32,
            ItemKind::LightBlueDye => 813u32,
            ItemKind::YellowDye => 814u32,
            ItemKind::LimeDye => 815u32,
            ItemKind::PinkDye => 816u32,
            ItemKind::GrayDye => 817u32,
            ItemKind::LightGrayDye => 818u32,
            ItemKind::CyanDye => 819u32,
            ItemKind::PurpleDye => 820u32,
            ItemKind::BlueDye => 821u32,
            ItemKind::BrownDye => 822u32,
            ItemKind::GreenDye => 823u32,
            ItemKind::RedDye => 824u32,
            ItemKind::BlackDye => 825u32,
            ItemKind::BoneMeal => 826u32,
            ItemKind::Bone => 827u32,
            ItemKind::Sugar => 828u32,
            ItemKind::Cake => 829u32,
            ItemKind::WhiteBed => 830u32,
            ItemKind::OrangeBed => 831u32,
            ItemKind::MagentaBed => 832u32,
            ItemKind::LightBlueBed => 833u32,
            ItemKind::YellowBed => 834u32,
            ItemKind::LimeBed => 835u32,
            ItemKind::PinkBed => 836u32,
            ItemKind::GrayBed => 837u32,
            ItemKind::LightGrayBed => 838u32,
            ItemKind::CyanBed => 839u32,
            ItemKind::PurpleBed => 840u32,
            ItemKind::BlueBed => 841u32,
            ItemKind::BrownBed => 842u32,
            ItemKind::GreenBed => 843u32,
            ItemKind::RedBed => 844u32,
            ItemKind::BlackBed => 845u32,
            ItemKind::Cookie => 846u32,
            ItemKind::FilledMap => 847u32,
            ItemKind::Shears => 848u32,
            ItemKind::MelonSlice => 849u32,
            ItemKind::DriedKelp => 850u32,
            ItemKind::PumpkinSeeds => 851u32,
            ItemKind::MelonSeeds => 852u32,
            ItemKind::Beef => 853u32,
            ItemKind::CookedBeef => 854u32,
            ItemKind::Chicken => 855u32,
            ItemKind::CookedChicken => 856u32,
            ItemKind::RottenFlesh => 857u32,
            ItemKind::EnderPearl => 858u32,
            ItemKind::BlazeRod => 859u32,
            ItemKind::GhastTear => 860u32,
            ItemKind::GoldNugget => 861u32,
            ItemKind::NetherWart => 862u32,
            ItemKind::Potion => 863u32,
            ItemKind::GlassBottle => 864u32,
            ItemKind::SpiderEye => 865u32,
            ItemKind::FermentedSpiderEye => 866u32,
            ItemKind::BlazePowder => 867u32,
            ItemKind::MagmaCream => 868u32,
            ItemKind::BrewingStand => 869u32,
            ItemKind::Cauldron => 870u32,
            ItemKind::EnderEye => 871u32,
            ItemKind::GlisteringMelonSlice => 872u32,
            ItemKind::AxolotlSpawnEgg => 873u32,
            ItemKind::BatSpawnEgg => 874u32,
            ItemKind::BeeSpawnEgg => 875u32,
            ItemKind::BlazeSpawnEgg => 876u32,
            ItemKind::CatSpawnEgg => 877u32,
            ItemKind::CaveSpiderSpawnEgg => 878u32,
            ItemKind::ChickenSpawnEgg => 879u32,
            ItemKind::CodSpawnEgg => 880u32,
            ItemKind::CowSpawnEgg => 881u32,
            ItemKind::CreeperSpawnEgg => 882u32,
            ItemKind::DolphinSpawnEgg => 883u32,
            ItemKind::DonkeySpawnEgg => 884u32,
            ItemKind::DrownedSpawnEgg => 885u32,
            ItemKind::ElderGuardianSpawnEgg => 886u32,
            ItemKind::EndermanSpawnEgg => 887u32,
            ItemKind::EndermiteSpawnEgg => 888u32,
            ItemKind::EvokerSpawnEgg => 889u32,
            ItemKind::FoxSpawnEgg => 890u32,
            ItemKind::GhastSpawnEgg => 891u32,
            ItemKind::GlowSquidSpawnEgg => 892u32,
            ItemKind::GoatSpawnEgg => 893u32,
            ItemKind::GuardianSpawnEgg => 894u32,
            ItemKind::HoglinSpawnEgg => 895u32,
            ItemKind::HorseSpawnEgg => 896u32,
            ItemKind::HuskSpawnEgg => 897u32,
            ItemKind::LlamaSpawnEgg => 898u32,
            ItemKind::MagmaCubeSpawnEgg => 899u32,
            ItemKind::MooshroomSpawnEgg => 900u32,
            ItemKind::MuleSpawnEgg => 901u32,
            ItemKind::OcelotSpawnEgg => 902u32,
            ItemKind::PandaSpawnEgg => 903u32,
            ItemKind::ParrotSpawnEgg => 904u32,
            ItemKind::PhantomSpawnEgg => 905u32,
            ItemKind::PigSpawnEgg => 906u32,
            ItemKind::PiglinSpawnEgg => 907u32,
            ItemKind::PiglinBruteSpawnEgg => 908u32,
            ItemKind::PillagerSpawnEgg => 909u32,
            ItemKind::PolarBearSpawnEgg => 910u32,
            ItemKind::PufferfishSpawnEgg => 911u32,
            ItemKind::RabbitSpawnEgg => 912u32,
            ItemKind::RavagerSpawnEgg => 913u32,
            ItemKind::SalmonSpawnEgg => 914u32,
            ItemKind::SheepSpawnEgg => 915u32,
            ItemKind::ShulkerSpawnEgg => 916u32,
            ItemKind::SilverfishSpawnEgg => 917u32,
            ItemKind::SkeletonSpawnEgg => 918u32,
            ItemKind::SkeletonHorseSpawnEgg => 919u32,
            ItemKind::SlimeSpawnEgg => 920u32,
            ItemKind::SpiderSpawnEgg => 921u32,
            ItemKind::SquidSpawnEgg => 922u32,
            ItemKind::StraySpawnEgg => 923u32,
            ItemKind::StriderSpawnEgg => 924u32,
            ItemKind::TraderLlamaSpawnEgg => 925u32,
            ItemKind::TropicalFishSpawnEgg => 926u32,
            ItemKind::TurtleSpawnEgg => 927u32,
            ItemKind::VexSpawnEgg => 928u32,
            ItemKind::VillagerSpawnEgg => 929u32,
            ItemKind::VindicatorSpawnEgg => 930u32,
            ItemKind::WanderingTraderSpawnEgg => 931u32,
            ItemKind::WitchSpawnEgg => 932u32,
            ItemKind::WitherSkeletonSpawnEgg => 933u32,
            ItemKind::WolfSpawnEgg => 934u32,
            ItemKind::ZoglinSpawnEgg => 935u32,
            ItemKind::ZombieSpawnEgg => 936u32,
            ItemKind::ZombieHorseSpawnEgg => 937u32,
            ItemKind::ZombieVillagerSpawnEgg => 938u32,
            ItemKind::ZombifiedPiglinSpawnEgg => 939u32,
            ItemKind::ExperienceBottle => 940u32,
            ItemKind::FireCharge => 941u32,
            ItemKind::WritableBook => 942u32,
            ItemKind::WrittenBook => 943u32,
            ItemKind::ItemFrame => 944u32,
            ItemKind::GlowItemFrame => 945u32,
            ItemKind::FlowerPot => 946u32,
            ItemKind::Carrot => 947u32,
            ItemKind::Potato => 948u32,
            ItemKind::BakedPotato => 949u32,
            ItemKind::PoisonousPotato => 950u32,
            ItemKind::Map => 951u32,
            ItemKind::GoldenCarrot => 952u32,
            ItemKind::SkeletonSkull => 953u32,
            ItemKind::WitherSkeletonSkull => 954u32,
            ItemKind::PlayerHead => 955u32,
            ItemKind::ZombieHead => 956u32,
            ItemKind::CreeperHead => 957u32,
            ItemKind::DragonHead => 958u32,
            ItemKind::NetherStar => 959u32,
            ItemKind::PumpkinPie => 960u32,
            ItemKind::FireworkRocket => 961u32,
            ItemKind::FireworkStar => 962u32,
            ItemKind::EnchantedBook => 963u32,
            ItemKind::NetherBrick => 964u32,
            ItemKind::PrismarineShard => 965u32,
            ItemKind::PrismarineCrystals => 966u32,
            ItemKind::Rabbit => 967u32,
            ItemKind::CookedRabbit => 968u32,
            ItemKind::RabbitStew => 969u32,
            ItemKind::RabbitFoot => 970u32,
            ItemKind::RabbitHide => 971u32,
            ItemKind::ArmorStand => 972u32,
            ItemKind::IronHorseArmor => 973u32,
            ItemKind::GoldenHorseArmor => 974u32,
            ItemKind::DiamondHorseArmor => 975u32,
            ItemKind::LeatherHorseArmor => 976u32,
            ItemKind::Lead => 977u32,
            ItemKind::NameTag => 978u32,
            ItemKind::CommandBlockMinecart => 979u32,
            ItemKind::Mutton => 980u32,
            ItemKind::CookedMutton => 981u32,
            ItemKind::WhiteBanner => 982u32,
            ItemKind::OrangeBanner => 983u32,
            ItemKind::MagentaBanner => 984u32,
            ItemKind::LightBlueBanner => 985u32,
            ItemKind::YellowBanner => 986u32,
            ItemKind::LimeBanner => 987u32,
            ItemKind::PinkBanner => 988u32,
            ItemKind::GrayBanner => 989u32,
            ItemKind::LightGrayBanner => 990u32,
            ItemKind::CyanBanner => 991u32,
            ItemKind::PurpleBanner => 992u32,
            ItemKind::BlueBanner => 993u32,
            ItemKind::BrownBanner => 994u32,
            ItemKind::GreenBanner => 995u32,
            ItemKind::RedBanner => 996u32,
            ItemKind::BlackBanner => 997u32,
            ItemKind::EndCrystal => 998u32,
            ItemKind::ChorusFruit => 999u32,
            ItemKind::PoppedChorusFruit => 1000u32,
            ItemKind::Beetroot => 1001u32,
            ItemKind::BeetrootSeeds => 1002u32,
            ItemKind::BeetrootSoup => 1003u32,
            ItemKind::DragonBreath => 1004u32,
            ItemKind::SplashPotion => 1005u32,
            ItemKind::SpectralArrow => 1006u32,
            ItemKind::TippedArrow => 1007u32,
            ItemKind::LingeringPotion => 1008u32,
            ItemKind::Shield => 1009u32,
            ItemKind::TotemOfUndying => 1010u32,
            ItemKind::ShulkerShell => 1011u32,
            ItemKind::IronNugget => 1012u32,
            ItemKind::KnowledgeBook => 1013u32,
            ItemKind::DebugStick => 1014u32,
            ItemKind::MusicDisc13 => 1015u32,
            ItemKind::MusicDiscCat => 1016u32,
            ItemKind::MusicDiscBlocks => 1017u32,
            ItemKind::MusicDiscChirp => 1018u32,
            ItemKind::MusicDiscFar => 1019u32,
            ItemKind::MusicDiscMall => 1020u32,
            ItemKind::MusicDiscMellohi => 1021u32,
            ItemKind::MusicDiscStal => 1022u32,
            ItemKind::MusicDiscStrad => 1023u32,
            ItemKind::MusicDiscWard => 1024u32,
            ItemKind::MusicDisc11 => 1025u32,
            ItemKind::MusicDiscWait => 1026u32,
            ItemKind::MusicDiscOtherside => 1027u32,
            ItemKind::MusicDiscPigstep => 1028u32,
            ItemKind::Trident => 1029u32,
            ItemKind::PhantomMembrane => 1030u32,
            ItemKind::NautilusShell => 1031u32,
            ItemKind::HeartOfTheSea => 1032u32,
            ItemKind::Crossbow => 1033u32,
            ItemKind::SuspiciousStew => 1034u32,
            ItemKind::Loom => 1035u32,
            ItemKind::FlowerBannerPattern => 1036u32,
            ItemKind::CreeperBannerPattern => 1037u32,
            ItemKind::SkullBannerPattern => 1038u32,
            ItemKind::MojangBannerPattern => 1039u32,
            ItemKind::GlobeBannerPattern => 1040u32,
            ItemKind::PiglinBannerPattern => 1041u32,
            ItemKind::Composter => 1042u32,
            ItemKind::Barrel => 1043u32,
            ItemKind::Smoker => 1044u32,
            ItemKind::BlastFurnace => 1045u32,
            ItemKind::CartographyTable => 1046u32,
            ItemKind::FletchingTable => 1047u32,
            ItemKind::Grindstone => 1048u32,
            ItemKind::SmithingTable => 1049u32,
            ItemKind::Stonecutter => 1050u32,
            ItemKind::Bell => 1051u32,
            ItemKind::Lantern => 1052u32,
            ItemKind::SoulLantern => 1053u32,
            ItemKind::SweetBerries => 1054u32,
            ItemKind::GlowBerries => 1055u32,
            ItemKind::Campfire => 1056u32,
            ItemKind::SoulCampfire => 1057u32,
            ItemKind::Shroomlight => 1058u32,
            ItemKind::Honeycomb => 1059u32,
            ItemKind::BeeNest => 1060u32,
            ItemKind::Beehive => 1061u32,
            ItemKind::HoneyBottle => 1062u32,
            ItemKind::HoneycombBlock => 1063u32,
            ItemKind::Lodestone => 1064u32,
            ItemKind::CryingObsidian => 1065u32,
            ItemKind::Blackstone => 1066u32,
            ItemKind::BlackstoneSlab => 1067u32,
            ItemKind::BlackstoneStairs => 1068u32,
            ItemKind::GildedBlackstone => 1069u32,
            ItemKind::PolishedBlackstone => 1070u32,
            ItemKind::PolishedBlackstoneSlab => 1071u32,
            ItemKind::PolishedBlackstoneStairs => 1072u32,
            ItemKind::ChiseledPolishedBlackstone => 1073u32,
            ItemKind::PolishedBlackstoneBricks => 1074u32,
            ItemKind::PolishedBlackstoneBrickSlab => 1075u32,
            ItemKind::PolishedBlackstoneBrickStairs => 1076u32,
            ItemKind::CrackedPolishedBlackstoneBricks => 1077u32,
            ItemKind::RespawnAnchor => 1078u32,
            ItemKind::Candle => 1079u32,
            ItemKind::WhiteCandle => 1080u32,
            ItemKind::OrangeCandle => 1081u32,
            ItemKind::MagentaCandle => 1082u32,
            ItemKind::LightBlueCandle => 1083u32,
            ItemKind::YellowCandle => 1084u32,
            ItemKind::LimeCandle => 1085u32,
            ItemKind::PinkCandle => 1086u32,
            ItemKind::GrayCandle => 1087u32,
            ItemKind::LightGrayCandle => 1088u32,
            ItemKind::CyanCandle => 1089u32,
            ItemKind::PurpleCandle => 1090u32,
            ItemKind::BlueCandle => 1091u32,
            ItemKind::BrownCandle => 1092u32,
            ItemKind::GreenCandle => 1093u32,
            ItemKind::RedCandle => 1094u32,
            ItemKind::BlackCandle => 1095u32,
            ItemKind::SmallAmethystBud => 1096u32,
            ItemKind::MediumAmethystBud => 1097u32,
            ItemKind::LargeAmethystBud => 1098u32,
            ItemKind::AmethystCluster => 1099u32,
            ItemKind::PointedDripstone => 1100u32,
        }
    }
    #[doc = "Gets a `ItemKind` by its `id`."]
    #[inline]
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            1u32 => Some(ItemKind::Stone),
            2u32 => Some(ItemKind::Granite),
            3u32 => Some(ItemKind::PolishedGranite),
            4u32 => Some(ItemKind::Diorite),
            5u32 => Some(ItemKind::PolishedDiorite),
            6u32 => Some(ItemKind::Andesite),
            7u32 => Some(ItemKind::PolishedAndesite),
            8u32 => Some(ItemKind::Deepslate),
            9u32 => Some(ItemKind::CobbledDeepslate),
            10u32 => Some(ItemKind::PolishedDeepslate),
            11u32 => Some(ItemKind::Calcite),
            12u32 => Some(ItemKind::Tuff),
            13u32 => Some(ItemKind::DripstoneBlock),
            14u32 => Some(ItemKind::GrassBlock),
            15u32 => Some(ItemKind::Dirt),
            16u32 => Some(ItemKind::CoarseDirt),
            17u32 => Some(ItemKind::Podzol),
            18u32 => Some(ItemKind::RootedDirt),
            19u32 => Some(ItemKind::CrimsonNylium),
            20u32 => Some(ItemKind::WarpedNylium),
            21u32 => Some(ItemKind::Cobblestone),
            22u32 => Some(ItemKind::OakPlanks),
            23u32 => Some(ItemKind::SprucePlanks),
            24u32 => Some(ItemKind::BirchPlanks),
            25u32 => Some(ItemKind::JunglePlanks),
            26u32 => Some(ItemKind::AcaciaPlanks),
            27u32 => Some(ItemKind::DarkOakPlanks),
            28u32 => Some(ItemKind::CrimsonPlanks),
            29u32 => Some(ItemKind::WarpedPlanks),
            30u32 => Some(ItemKind::OakSapling),
            31u32 => Some(ItemKind::SpruceSapling),
            32u32 => Some(ItemKind::BirchSapling),
            33u32 => Some(ItemKind::JungleSapling),
            34u32 => Some(ItemKind::AcaciaSapling),
            35u32 => Some(ItemKind::DarkOakSapling),
            36u32 => Some(ItemKind::Bedrock),
            37u32 => Some(ItemKind::Sand),
            38u32 => Some(ItemKind::RedSand),
            39u32 => Some(ItemKind::Gravel),
            40u32 => Some(ItemKind::CoalOre),
            41u32 => Some(ItemKind::DeepslateCoalOre),
            42u32 => Some(ItemKind::IronOre),
            43u32 => Some(ItemKind::DeepslateIronOre),
            44u32 => Some(ItemKind::CopperOre),
            45u32 => Some(ItemKind::DeepslateCopperOre),
            46u32 => Some(ItemKind::GoldOre),
            47u32 => Some(ItemKind::DeepslateGoldOre),
            48u32 => Some(ItemKind::RedstoneOre),
            49u32 => Some(ItemKind::DeepslateRedstoneOre),
            50u32 => Some(ItemKind::EmeraldOre),
            51u32 => Some(ItemKind::DeepslateEmeraldOre),
            52u32 => Some(ItemKind::LapisOre),
            53u32 => Some(ItemKind::DeepslateLapisOre),
            54u32 => Some(ItemKind::DiamondOre),
            55u32 => Some(ItemKind::DeepslateDiamondOre),
            56u32 => Some(ItemKind::NetherGoldOre),
            57u32 => Some(ItemKind::NetherQuartzOre),
            58u32 => Some(ItemKind::AncientDebris),
            59u32 => Some(ItemKind::CoalBlock),
            60u32 => Some(ItemKind::RawIronBlock),
            61u32 => Some(ItemKind::RawCopperBlock),
            62u32 => Some(ItemKind::RawGoldBlock),
            63u32 => Some(ItemKind::AmethystBlock),
            64u32 => Some(ItemKind::BuddingAmethyst),
            65u32 => Some(ItemKind::IronBlock),
            66u32 => Some(ItemKind::CopperBlock),
            67u32 => Some(ItemKind::GoldBlock),
            68u32 => Some(ItemKind::DiamondBlock),
            69u32 => Some(ItemKind::NetheriteBlock),
            70u32 => Some(ItemKind::ExposedCopper),
            71u32 => Some(ItemKind::WeatheredCopper),
            72u32 => Some(ItemKind::OxidizedCopper),
            73u32 => Some(ItemKind::CutCopper),
            74u32 => Some(ItemKind::ExposedCutCopper),
            75u32 => Some(ItemKind::WeatheredCutCopper),
            76u32 => Some(ItemKind::OxidizedCutCopper),
            77u32 => Some(ItemKind::CutCopperStairs),
            78u32 => Some(ItemKind::ExposedCutCopperStairs),
            79u32 => Some(ItemKind::WeatheredCutCopperStairs),
            80u32 => Some(ItemKind::OxidizedCutCopperStairs),
            81u32 => Some(ItemKind::CutCopperSlab),
            82u32 => Some(ItemKind::ExposedCutCopperSlab),
            83u32 => Some(ItemKind::WeatheredCutCopperSlab),
            84u32 => Some(ItemKind::OxidizedCutCopperSlab),
            85u32 => Some(ItemKind::WaxedCopperBlock),
            86u32 => Some(ItemKind::WaxedExposedCopper),
            87u32 => Some(ItemKind::WaxedWeatheredCopper),
            88u32 => Some(ItemKind::WaxedOxidizedCopper),
            89u32 => Some(ItemKind::WaxedCutCopper),
            90u32 => Some(ItemKind::WaxedExposedCutCopper),
            91u32 => Some(ItemKind::WaxedWeatheredCutCopper),
            92u32 => Some(ItemKind::WaxedOxidizedCutCopper),
            93u32 => Some(ItemKind::WaxedCutCopperStairs),
            94u32 => Some(ItemKind::WaxedExposedCutCopperStairs),
            95u32 => Some(ItemKind::WaxedWeatheredCutCopperStairs),
            96u32 => Some(ItemKind::WaxedOxidizedCutCopperStairs),
            97u32 => Some(ItemKind::WaxedCutCopperSlab),
            98u32 => Some(ItemKind::WaxedExposedCutCopperSlab),
            99u32 => Some(ItemKind::WaxedWeatheredCutCopperSlab),
            100u32 => Some(ItemKind::WaxedOxidizedCutCopperSlab),
            101u32 => Some(ItemKind::OakLog),
            102u32 => Some(ItemKind::SpruceLog),
            103u32 => Some(ItemKind::BirchLog),
            104u32 => Some(ItemKind::JungleLog),
            105u32 => Some(ItemKind::AcaciaLog),
            106u32 => Some(ItemKind::DarkOakLog),
            107u32 => Some(ItemKind::CrimsonStem),
            108u32 => Some(ItemKind::WarpedStem),
            109u32 => Some(ItemKind::StrippedOakLog),
            110u32 => Some(ItemKind::StrippedSpruceLog),
            111u32 => Some(ItemKind::StrippedBirchLog),
            112u32 => Some(ItemKind::StrippedJungleLog),
            113u32 => Some(ItemKind::StrippedAcaciaLog),
            114u32 => Some(ItemKind::StrippedDarkOakLog),
            115u32 => Some(ItemKind::StrippedCrimsonStem),
            116u32 => Some(ItemKind::StrippedWarpedStem),
            117u32 => Some(ItemKind::StrippedOakWood),
            118u32 => Some(ItemKind::StrippedSpruceWood),
            119u32 => Some(ItemKind::StrippedBirchWood),
            120u32 => Some(ItemKind::StrippedJungleWood),
            121u32 => Some(ItemKind::StrippedAcaciaWood),
            122u32 => Some(ItemKind::StrippedDarkOakWood),
            123u32 => Some(ItemKind::StrippedCrimsonHyphae),
            124u32 => Some(ItemKind::StrippedWarpedHyphae),
            125u32 => Some(ItemKind::OakWood),
            126u32 => Some(ItemKind::SpruceWood),
            127u32 => Some(ItemKind::BirchWood),
            128u32 => Some(ItemKind::JungleWood),
            129u32 => Some(ItemKind::AcaciaWood),
            130u32 => Some(ItemKind::DarkOakWood),
            131u32 => Some(ItemKind::CrimsonHyphae),
            132u32 => Some(ItemKind::WarpedHyphae),
            133u32 => Some(ItemKind::OakLeaves),
            134u32 => Some(ItemKind::SpruceLeaves),
            135u32 => Some(ItemKind::BirchLeaves),
            136u32 => Some(ItemKind::JungleLeaves),
            137u32 => Some(ItemKind::AcaciaLeaves),
            138u32 => Some(ItemKind::DarkOakLeaves),
            139u32 => Some(ItemKind::AzaleaLeaves),
            140u32 => Some(ItemKind::FloweringAzaleaLeaves),
            141u32 => Some(ItemKind::Sponge),
            142u32 => Some(ItemKind::WetSponge),
            143u32 => Some(ItemKind::Glass),
            144u32 => Some(ItemKind::TintedGlass),
            145u32 => Some(ItemKind::LapisBlock),
            146u32 => Some(ItemKind::Sandstone),
            147u32 => Some(ItemKind::ChiseledSandstone),
            148u32 => Some(ItemKind::CutSandstone),
            149u32 => Some(ItemKind::Cobweb),
            150u32 => Some(ItemKind::Grass),
            151u32 => Some(ItemKind::Fern),
            152u32 => Some(ItemKind::Azalea),
            153u32 => Some(ItemKind::FloweringAzalea),
            154u32 => Some(ItemKind::DeadBush),
            155u32 => Some(ItemKind::Seagrass),
            156u32 => Some(ItemKind::SeaPickle),
            157u32 => Some(ItemKind::WhiteWool),
            158u32 => Some(ItemKind::OrangeWool),
            159u32 => Some(ItemKind::MagentaWool),
            160u32 => Some(ItemKind::LightBlueWool),
            161u32 => Some(ItemKind::YellowWool),
            162u32 => Some(ItemKind::LimeWool),
            163u32 => Some(ItemKind::PinkWool),
            164u32 => Some(ItemKind::GrayWool),
            165u32 => Some(ItemKind::LightGrayWool),
            166u32 => Some(ItemKind::CyanWool),
            167u32 => Some(ItemKind::PurpleWool),
            168u32 => Some(ItemKind::BlueWool),
            169u32 => Some(ItemKind::BrownWool),
            170u32 => Some(ItemKind::GreenWool),
            171u32 => Some(ItemKind::RedWool),
            172u32 => Some(ItemKind::BlackWool),
            173u32 => Some(ItemKind::Dandelion),
            174u32 => Some(ItemKind::Poppy),
            175u32 => Some(ItemKind::BlueOrchid),
            176u32 => Some(ItemKind::Allium),
            177u32 => Some(ItemKind::AzureBluet),
            178u32 => Some(ItemKind::RedTulip),
            179u32 => Some(ItemKind::OrangeTulip),
            180u32 => Some(ItemKind::WhiteTulip),
            181u32 => Some(ItemKind::PinkTulip),
            182u32 => Some(ItemKind::OxeyeDaisy),
            183u32 => Some(ItemKind::Cornflower),
            184u32 => Some(ItemKind::LilyOfTheValley),
            185u32 => Some(ItemKind::WitherRose),
            186u32 => Some(ItemKind::SporeBlossom),
            187u32 => Some(ItemKind::BrownMushroom),
            188u32 => Some(ItemKind::RedMushroom),
            189u32 => Some(ItemKind::CrimsonFungus),
            190u32 => Some(ItemKind::WarpedFungus),
            191u32 => Some(ItemKind::CrimsonRoots),
            192u32 => Some(ItemKind::WarpedRoots),
            193u32 => Some(ItemKind::NetherSprouts),
            194u32 => Some(ItemKind::WeepingVines),
            195u32 => Some(ItemKind::TwistingVines),
            196u32 => Some(ItemKind::SugarCane),
            197u32 => Some(ItemKind::Kelp),
            198u32 => Some(ItemKind::MossCarpet),
            199u32 => Some(ItemKind::MossBlock),
            200u32 => Some(ItemKind::HangingRoots),
            201u32 => Some(ItemKind::BigDripleaf),
            202u32 => Some(ItemKind::SmallDripleaf),
            203u32 => Some(ItemKind::Bamboo),
            204u32 => Some(ItemKind::OakSlab),
            205u32 => Some(ItemKind::SpruceSlab),
            206u32 => Some(ItemKind::BirchSlab),
            207u32 => Some(ItemKind::JungleSlab),
            208u32 => Some(ItemKind::AcaciaSlab),
            209u32 => Some(ItemKind::DarkOakSlab),
            210u32 => Some(ItemKind::CrimsonSlab),
            211u32 => Some(ItemKind::WarpedSlab),
            212u32 => Some(ItemKind::StoneSlab),
            213u32 => Some(ItemKind::SmoothStoneSlab),
            214u32 => Some(ItemKind::SandstoneSlab),
            215u32 => Some(ItemKind::CutSandstoneSlab),
            216u32 => Some(ItemKind::PetrifiedOakSlab),
            217u32 => Some(ItemKind::CobblestoneSlab),
            218u32 => Some(ItemKind::BrickSlab),
            219u32 => Some(ItemKind::StoneBrickSlab),
            220u32 => Some(ItemKind::NetherBrickSlab),
            221u32 => Some(ItemKind::QuartzSlab),
            222u32 => Some(ItemKind::RedSandstoneSlab),
            223u32 => Some(ItemKind::CutRedSandstoneSlab),
            224u32 => Some(ItemKind::PurpurSlab),
            225u32 => Some(ItemKind::PrismarineSlab),
            226u32 => Some(ItemKind::PrismarineBrickSlab),
            227u32 => Some(ItemKind::DarkPrismarineSlab),
            228u32 => Some(ItemKind::SmoothQuartz),
            229u32 => Some(ItemKind::SmoothRedSandstone),
            230u32 => Some(ItemKind::SmoothSandstone),
            231u32 => Some(ItemKind::SmoothStone),
            232u32 => Some(ItemKind::Bricks),
            233u32 => Some(ItemKind::Bookshelf),
            234u32 => Some(ItemKind::MossyCobblestone),
            235u32 => Some(ItemKind::Obsidian),
            236u32 => Some(ItemKind::Torch),
            237u32 => Some(ItemKind::EndRod),
            238u32 => Some(ItemKind::ChorusPlant),
            239u32 => Some(ItemKind::ChorusFlower),
            240u32 => Some(ItemKind::PurpurBlock),
            241u32 => Some(ItemKind::PurpurPillar),
            242u32 => Some(ItemKind::PurpurStairs),
            243u32 => Some(ItemKind::Spawner),
            244u32 => Some(ItemKind::OakStairs),
            245u32 => Some(ItemKind::Chest),
            246u32 => Some(ItemKind::CraftingTable),
            247u32 => Some(ItemKind::Farmland),
            248u32 => Some(ItemKind::Furnace),
            249u32 => Some(ItemKind::Ladder),
            250u32 => Some(ItemKind::CobblestoneStairs),
            251u32 => Some(ItemKind::Snow),
            252u32 => Some(ItemKind::Ice),
            253u32 => Some(ItemKind::SnowBlock),
            254u32 => Some(ItemKind::Cactus),
            255u32 => Some(ItemKind::Clay),
            256u32 => Some(ItemKind::Jukebox),
            257u32 => Some(ItemKind::OakFence),
            258u32 => Some(ItemKind::SpruceFence),
            259u32 => Some(ItemKind::BirchFence),
            260u32 => Some(ItemKind::JungleFence),
            261u32 => Some(ItemKind::AcaciaFence),
            262u32 => Some(ItemKind::DarkOakFence),
            263u32 => Some(ItemKind::CrimsonFence),
            264u32 => Some(ItemKind::WarpedFence),
            265u32 => Some(ItemKind::Pumpkin),
            266u32 => Some(ItemKind::CarvedPumpkin),
            267u32 => Some(ItemKind::JackOLantern),
            268u32 => Some(ItemKind::Netherrack),
            269u32 => Some(ItemKind::SoulSand),
            270u32 => Some(ItemKind::SoulSoil),
            271u32 => Some(ItemKind::Basalt),
            272u32 => Some(ItemKind::PolishedBasalt),
            273u32 => Some(ItemKind::SmoothBasalt),
            274u32 => Some(ItemKind::SoulTorch),
            275u32 => Some(ItemKind::Glowstone),
            276u32 => Some(ItemKind::InfestedStone),
            277u32 => Some(ItemKind::InfestedCobblestone),
            278u32 => Some(ItemKind::InfestedStoneBricks),
            279u32 => Some(ItemKind::InfestedMossyStoneBricks),
            280u32 => Some(ItemKind::InfestedCrackedStoneBricks),
            281u32 => Some(ItemKind::InfestedChiseledStoneBricks),
            282u32 => Some(ItemKind::InfestedDeepslate),
            283u32 => Some(ItemKind::StoneBricks),
            284u32 => Some(ItemKind::MossyStoneBricks),
            285u32 => Some(ItemKind::CrackedStoneBricks),
            286u32 => Some(ItemKind::ChiseledStoneBricks),
            287u32 => Some(ItemKind::DeepslateBricks),
            288u32 => Some(ItemKind::CrackedDeepslateBricks),
            289u32 => Some(ItemKind::DeepslateTiles),
            290u32 => Some(ItemKind::CrackedDeepslateTiles),
            291u32 => Some(ItemKind::ChiseledDeepslate),
            292u32 => Some(ItemKind::BrownMushroomBlock),
            293u32 => Some(ItemKind::RedMushroomBlock),
            294u32 => Some(ItemKind::MushroomStem),
            295u32 => Some(ItemKind::IronBars),
            296u32 => Some(ItemKind::Chain),
            297u32 => Some(ItemKind::GlassPane),
            298u32 => Some(ItemKind::Melon),
            299u32 => Some(ItemKind::Vine),
            300u32 => Some(ItemKind::GlowLichen),
            301u32 => Some(ItemKind::BrickStairs),
            302u32 => Some(ItemKind::StoneBrickStairs),
            303u32 => Some(ItemKind::Mycelium),
            304u32 => Some(ItemKind::LilyPad),
            305u32 => Some(ItemKind::NetherBricks),
            306u32 => Some(ItemKind::CrackedNetherBricks),
            307u32 => Some(ItemKind::ChiseledNetherBricks),
            308u32 => Some(ItemKind::NetherBrickFence),
            309u32 => Some(ItemKind::NetherBrickStairs),
            310u32 => Some(ItemKind::EnchantingTable),
            311u32 => Some(ItemKind::EndPortalFrame),
            312u32 => Some(ItemKind::EndStone),
            313u32 => Some(ItemKind::EndStoneBricks),
            314u32 => Some(ItemKind::DragonEgg),
            315u32 => Some(ItemKind::SandstoneStairs),
            316u32 => Some(ItemKind::EnderChest),
            317u32 => Some(ItemKind::EmeraldBlock),
            318u32 => Some(ItemKind::SpruceStairs),
            319u32 => Some(ItemKind::BirchStairs),
            320u32 => Some(ItemKind::JungleStairs),
            321u32 => Some(ItemKind::CrimsonStairs),
            322u32 => Some(ItemKind::WarpedStairs),
            323u32 => Some(ItemKind::CommandBlock),
            324u32 => Some(ItemKind::Beacon),
            325u32 => Some(ItemKind::CobblestoneWall),
            326u32 => Some(ItemKind::MossyCobblestoneWall),
            327u32 => Some(ItemKind::BrickWall),
            328u32 => Some(ItemKind::PrismarineWall),
            329u32 => Some(ItemKind::RedSandstoneWall),
            330u32 => Some(ItemKind::MossyStoneBrickWall),
            331u32 => Some(ItemKind::GraniteWall),
            332u32 => Some(ItemKind::StoneBrickWall),
            333u32 => Some(ItemKind::NetherBrickWall),
            334u32 => Some(ItemKind::AndesiteWall),
            335u32 => Some(ItemKind::RedNetherBrickWall),
            336u32 => Some(ItemKind::SandstoneWall),
            337u32 => Some(ItemKind::EndStoneBrickWall),
            338u32 => Some(ItemKind::DioriteWall),
            339u32 => Some(ItemKind::BlackstoneWall),
            340u32 => Some(ItemKind::PolishedBlackstoneWall),
            341u32 => Some(ItemKind::PolishedBlackstoneBrickWall),
            342u32 => Some(ItemKind::CobbledDeepslateWall),
            343u32 => Some(ItemKind::PolishedDeepslateWall),
            344u32 => Some(ItemKind::DeepslateBrickWall),
            345u32 => Some(ItemKind::DeepslateTileWall),
            346u32 => Some(ItemKind::Anvil),
            347u32 => Some(ItemKind::ChippedAnvil),
            348u32 => Some(ItemKind::DamagedAnvil),
            349u32 => Some(ItemKind::ChiseledQuartzBlock),
            350u32 => Some(ItemKind::QuartzBlock),
            351u32 => Some(ItemKind::QuartzBricks),
            352u32 => Some(ItemKind::QuartzPillar),
            353u32 => Some(ItemKind::QuartzStairs),
            354u32 => Some(ItemKind::WhiteTerracotta),
            355u32 => Some(ItemKind::OrangeTerracotta),
            356u32 => Some(ItemKind::MagentaTerracotta),
            357u32 => Some(ItemKind::LightBlueTerracotta),
            358u32 => Some(ItemKind::YellowTerracotta),
            359u32 => Some(ItemKind::LimeTerracotta),
            360u32 => Some(ItemKind::PinkTerracotta),
            361u32 => Some(ItemKind::GrayTerracotta),
            362u32 => Some(ItemKind::LightGrayTerracotta),
            363u32 => Some(ItemKind::CyanTerracotta),
            364u32 => Some(ItemKind::PurpleTerracotta),
            365u32 => Some(ItemKind::BlueTerracotta),
            366u32 => Some(ItemKind::BrownTerracotta),
            367u32 => Some(ItemKind::GreenTerracotta),
            368u32 => Some(ItemKind::RedTerracotta),
            369u32 => Some(ItemKind::BlackTerracotta),
            370u32 => Some(ItemKind::Barrier),
            371u32 => Some(ItemKind::Light),
            372u32 => Some(ItemKind::HayBlock),
            373u32 => Some(ItemKind::WhiteCarpet),
            374u32 => Some(ItemKind::OrangeCarpet),
            375u32 => Some(ItemKind::MagentaCarpet),
            376u32 => Some(ItemKind::LightBlueCarpet),
            377u32 => Some(ItemKind::YellowCarpet),
            378u32 => Some(ItemKind::LimeCarpet),
            379u32 => Some(ItemKind::PinkCarpet),
            380u32 => Some(ItemKind::GrayCarpet),
            381u32 => Some(ItemKind::LightGrayCarpet),
            382u32 => Some(ItemKind::CyanCarpet),
            383u32 => Some(ItemKind::PurpleCarpet),
            384u32 => Some(ItemKind::BlueCarpet),
            385u32 => Some(ItemKind::BrownCarpet),
            386u32 => Some(ItemKind::GreenCarpet),
            387u32 => Some(ItemKind::RedCarpet),
            388u32 => Some(ItemKind::BlackCarpet),
            389u32 => Some(ItemKind::Terracotta),
            390u32 => Some(ItemKind::PackedIce),
            391u32 => Some(ItemKind::AcaciaStairs),
            392u32 => Some(ItemKind::DarkOakStairs),
            393u32 => Some(ItemKind::DirtPath),
            394u32 => Some(ItemKind::Sunflower),
            395u32 => Some(ItemKind::Lilac),
            396u32 => Some(ItemKind::RoseBush),
            397u32 => Some(ItemKind::Peony),
            398u32 => Some(ItemKind::TallGrass),
            399u32 => Some(ItemKind::LargeFern),
            400u32 => Some(ItemKind::WhiteStainedGlass),
            401u32 => Some(ItemKind::OrangeStainedGlass),
            402u32 => Some(ItemKind::MagentaStainedGlass),
            403u32 => Some(ItemKind::LightBlueStainedGlass),
            404u32 => Some(ItemKind::YellowStainedGlass),
            405u32 => Some(ItemKind::LimeStainedGlass),
            406u32 => Some(ItemKind::PinkStainedGlass),
            407u32 => Some(ItemKind::GrayStainedGlass),
            408u32 => Some(ItemKind::LightGrayStainedGlass),
            409u32 => Some(ItemKind::CyanStainedGlass),
            410u32 => Some(ItemKind::PurpleStainedGlass),
            411u32 => Some(ItemKind::BlueStainedGlass),
            412u32 => Some(ItemKind::BrownStainedGlass),
            413u32 => Some(ItemKind::GreenStainedGlass),
            414u32 => Some(ItemKind::RedStainedGlass),
            415u32 => Some(ItemKind::BlackStainedGlass),
            416u32 => Some(ItemKind::WhiteStainedGlassPane),
            417u32 => Some(ItemKind::OrangeStainedGlassPane),
            418u32 => Some(ItemKind::MagentaStainedGlassPane),
            419u32 => Some(ItemKind::LightBlueStainedGlassPane),
            420u32 => Some(ItemKind::YellowStainedGlassPane),
            421u32 => Some(ItemKind::LimeStainedGlassPane),
            422u32 => Some(ItemKind::PinkStainedGlassPane),
            423u32 => Some(ItemKind::GrayStainedGlassPane),
            424u32 => Some(ItemKind::LightGrayStainedGlassPane),
            425u32 => Some(ItemKind::CyanStainedGlassPane),
            426u32 => Some(ItemKind::PurpleStainedGlassPane),
            427u32 => Some(ItemKind::BlueStainedGlassPane),
            428u32 => Some(ItemKind::BrownStainedGlassPane),
            429u32 => Some(ItemKind::GreenStainedGlassPane),
            430u32 => Some(ItemKind::RedStainedGlassPane),
            431u32 => Some(ItemKind::BlackStainedGlassPane),
            432u32 => Some(ItemKind::Prismarine),
            433u32 => Some(ItemKind::PrismarineBricks),
            434u32 => Some(ItemKind::DarkPrismarine),
            435u32 => Some(ItemKind::PrismarineStairs),
            436u32 => Some(ItemKind::PrismarineBrickStairs),
            437u32 => Some(ItemKind::DarkPrismarineStairs),
            438u32 => Some(ItemKind::SeaLantern),
            439u32 => Some(ItemKind::RedSandstone),
            440u32 => Some(ItemKind::ChiseledRedSandstone),
            441u32 => Some(ItemKind::CutRedSandstone),
            442u32 => Some(ItemKind::RedSandstoneStairs),
            443u32 => Some(ItemKind::RepeatingCommandBlock),
            444u32 => Some(ItemKind::ChainCommandBlock),
            445u32 => Some(ItemKind::MagmaBlock),
            446u32 => Some(ItemKind::NetherWartBlock),
            447u32 => Some(ItemKind::WarpedWartBlock),
            448u32 => Some(ItemKind::RedNetherBricks),
            449u32 => Some(ItemKind::BoneBlock),
            450u32 => Some(ItemKind::StructureVoid),
            451u32 => Some(ItemKind::ShulkerBox),
            452u32 => Some(ItemKind::WhiteShulkerBox),
            453u32 => Some(ItemKind::OrangeShulkerBox),
            454u32 => Some(ItemKind::MagentaShulkerBox),
            455u32 => Some(ItemKind::LightBlueShulkerBox),
            456u32 => Some(ItemKind::YellowShulkerBox),
            457u32 => Some(ItemKind::LimeShulkerBox),
            458u32 => Some(ItemKind::PinkShulkerBox),
            459u32 => Some(ItemKind::GrayShulkerBox),
            460u32 => Some(ItemKind::LightGrayShulkerBox),
            461u32 => Some(ItemKind::CyanShulkerBox),
            462u32 => Some(ItemKind::PurpleShulkerBox),
            463u32 => Some(ItemKind::BlueShulkerBox),
            464u32 => Some(ItemKind::BrownShulkerBox),
            465u32 => Some(ItemKind::GreenShulkerBox),
            466u32 => Some(ItemKind::RedShulkerBox),
            467u32 => Some(ItemKind::BlackShulkerBox),
            468u32 => Some(ItemKind::WhiteGlazedTerracotta),
            469u32 => Some(ItemKind::OrangeGlazedTerracotta),
            470u32 => Some(ItemKind::MagentaGlazedTerracotta),
            471u32 => Some(ItemKind::LightBlueGlazedTerracotta),
            472u32 => Some(ItemKind::YellowGlazedTerracotta),
            473u32 => Some(ItemKind::LimeGlazedTerracotta),
            474u32 => Some(ItemKind::PinkGlazedTerracotta),
            475u32 => Some(ItemKind::GrayGlazedTerracotta),
            476u32 => Some(ItemKind::LightGrayGlazedTerracotta),
            477u32 => Some(ItemKind::CyanGlazedTerracotta),
            478u32 => Some(ItemKind::PurpleGlazedTerracotta),
            479u32 => Some(ItemKind::BlueGlazedTerracotta),
            480u32 => Some(ItemKind::BrownGlazedTerracotta),
            481u32 => Some(ItemKind::GreenGlazedTerracotta),
            482u32 => Some(ItemKind::RedGlazedTerracotta),
            483u32 => Some(ItemKind::BlackGlazedTerracotta),
            484u32 => Some(ItemKind::WhiteConcrete),
            485u32 => Some(ItemKind::OrangeConcrete),
            486u32 => Some(ItemKind::MagentaConcrete),
            487u32 => Some(ItemKind::LightBlueConcrete),
            488u32 => Some(ItemKind::YellowConcrete),
            489u32 => Some(ItemKind::LimeConcrete),
            490u32 => Some(ItemKind::PinkConcrete),
            491u32 => Some(ItemKind::GrayConcrete),
            492u32 => Some(ItemKind::LightGrayConcrete),
            493u32 => Some(ItemKind::CyanConcrete),
            494u32 => Some(ItemKind::PurpleConcrete),
            495u32 => Some(ItemKind::BlueConcrete),
            496u32 => Some(ItemKind::BrownConcrete),
            497u32 => Some(ItemKind::GreenConcrete),
            498u32 => Some(ItemKind::RedConcrete),
            499u32 => Some(ItemKind::BlackConcrete),
            500u32 => Some(ItemKind::WhiteConcretePowder),
            501u32 => Some(ItemKind::OrangeConcretePowder),
            502u32 => Some(ItemKind::MagentaConcretePowder),
            503u32 => Some(ItemKind::LightBlueConcretePowder),
            504u32 => Some(ItemKind::YellowConcretePowder),
            505u32 => Some(ItemKind::LimeConcretePowder),
            506u32 => Some(ItemKind::PinkConcretePowder),
            507u32 => Some(ItemKind::GrayConcretePowder),
            508u32 => Some(ItemKind::LightGrayConcretePowder),
            509u32 => Some(ItemKind::CyanConcretePowder),
            510u32 => Some(ItemKind::PurpleConcretePowder),
            511u32 => Some(ItemKind::BlueConcretePowder),
            512u32 => Some(ItemKind::BrownConcretePowder),
            513u32 => Some(ItemKind::GreenConcretePowder),
            514u32 => Some(ItemKind::RedConcretePowder),
            515u32 => Some(ItemKind::BlackConcretePowder),
            516u32 => Some(ItemKind::TurtleEgg),
            517u32 => Some(ItemKind::DeadTubeCoralBlock),
            518u32 => Some(ItemKind::DeadBrainCoralBlock),
            519u32 => Some(ItemKind::DeadBubbleCoralBlock),
            520u32 => Some(ItemKind::DeadFireCoralBlock),
            521u32 => Some(ItemKind::DeadHornCoralBlock),
            522u32 => Some(ItemKind::TubeCoralBlock),
            523u32 => Some(ItemKind::BrainCoralBlock),
            524u32 => Some(ItemKind::BubbleCoralBlock),
            525u32 => Some(ItemKind::FireCoralBlock),
            526u32 => Some(ItemKind::HornCoralBlock),
            527u32 => Some(ItemKind::TubeCoral),
            528u32 => Some(ItemKind::BrainCoral),
            529u32 => Some(ItemKind::BubbleCoral),
            530u32 => Some(ItemKind::FireCoral),
            531u32 => Some(ItemKind::HornCoral),
            532u32 => Some(ItemKind::DeadBrainCoral),
            533u32 => Some(ItemKind::DeadBubbleCoral),
            534u32 => Some(ItemKind::DeadFireCoral),
            535u32 => Some(ItemKind::DeadHornCoral),
            536u32 => Some(ItemKind::DeadTubeCoral),
            537u32 => Some(ItemKind::TubeCoralFan),
            538u32 => Some(ItemKind::BrainCoralFan),
            539u32 => Some(ItemKind::BubbleCoralFan),
            540u32 => Some(ItemKind::FireCoralFan),
            541u32 => Some(ItemKind::HornCoralFan),
            542u32 => Some(ItemKind::DeadTubeCoralFan),
            543u32 => Some(ItemKind::DeadBrainCoralFan),
            544u32 => Some(ItemKind::DeadBubbleCoralFan),
            545u32 => Some(ItemKind::DeadFireCoralFan),
            546u32 => Some(ItemKind::DeadHornCoralFan),
            547u32 => Some(ItemKind::BlueIce),
            548u32 => Some(ItemKind::Conduit),
            549u32 => Some(ItemKind::PolishedGraniteStairs),
            550u32 => Some(ItemKind::SmoothRedSandstoneStairs),
            551u32 => Some(ItemKind::MossyStoneBrickStairs),
            552u32 => Some(ItemKind::PolishedDioriteStairs),
            553u32 => Some(ItemKind::MossyCobblestoneStairs),
            554u32 => Some(ItemKind::EndStoneBrickStairs),
            555u32 => Some(ItemKind::StoneStairs),
            556u32 => Some(ItemKind::SmoothSandstoneStairs),
            557u32 => Some(ItemKind::SmoothQuartzStairs),
            558u32 => Some(ItemKind::GraniteStairs),
            559u32 => Some(ItemKind::AndesiteStairs),
            560u32 => Some(ItemKind::RedNetherBrickStairs),
            561u32 => Some(ItemKind::PolishedAndesiteStairs),
            562u32 => Some(ItemKind::DioriteStairs),
            563u32 => Some(ItemKind::CobbledDeepslateStairs),
            564u32 => Some(ItemKind::PolishedDeepslateStairs),
            565u32 => Some(ItemKind::DeepslateBrickStairs),
            566u32 => Some(ItemKind::DeepslateTileStairs),
            567u32 => Some(ItemKind::PolishedGraniteSlab),
            568u32 => Some(ItemKind::SmoothRedSandstoneSlab),
            569u32 => Some(ItemKind::MossyStoneBrickSlab),
            570u32 => Some(ItemKind::PolishedDioriteSlab),
            571u32 => Some(ItemKind::MossyCobblestoneSlab),
            572u32 => Some(ItemKind::EndStoneBrickSlab),
            573u32 => Some(ItemKind::SmoothSandstoneSlab),
            574u32 => Some(ItemKind::SmoothQuartzSlab),
            575u32 => Some(ItemKind::GraniteSlab),
            576u32 => Some(ItemKind::AndesiteSlab),
            577u32 => Some(ItemKind::RedNetherBrickSlab),
            578u32 => Some(ItemKind::PolishedAndesiteSlab),
            579u32 => Some(ItemKind::DioriteSlab),
            580u32 => Some(ItemKind::CobbledDeepslateSlab),
            581u32 => Some(ItemKind::PolishedDeepslateSlab),
            582u32 => Some(ItemKind::DeepslateBrickSlab),
            583u32 => Some(ItemKind::DeepslateTileSlab),
            584u32 => Some(ItemKind::Scaffolding),
            585u32 => Some(ItemKind::Redstone),
            586u32 => Some(ItemKind::RedstoneTorch),
            587u32 => Some(ItemKind::RedstoneBlock),
            588u32 => Some(ItemKind::Repeater),
            589u32 => Some(ItemKind::Comparator),
            590u32 => Some(ItemKind::Piston),
            591u32 => Some(ItemKind::StickyPiston),
            592u32 => Some(ItemKind::SlimeBlock),
            593u32 => Some(ItemKind::HoneyBlock),
            594u32 => Some(ItemKind::Observer),
            595u32 => Some(ItemKind::Hopper),
            596u32 => Some(ItemKind::Dispenser),
            597u32 => Some(ItemKind::Dropper),
            598u32 => Some(ItemKind::Lectern),
            599u32 => Some(ItemKind::Target),
            600u32 => Some(ItemKind::Lever),
            601u32 => Some(ItemKind::LightningRod),
            602u32 => Some(ItemKind::DaylightDetector),
            603u32 => Some(ItemKind::SculkSensor),
            604u32 => Some(ItemKind::TripwireHook),
            605u32 => Some(ItemKind::TrappedChest),
            606u32 => Some(ItemKind::Tnt),
            607u32 => Some(ItemKind::RedstoneLamp),
            608u32 => Some(ItemKind::NoteBlock),
            609u32 => Some(ItemKind::StoneButton),
            610u32 => Some(ItemKind::PolishedBlackstoneButton),
            611u32 => Some(ItemKind::OakButton),
            612u32 => Some(ItemKind::SpruceButton),
            613u32 => Some(ItemKind::BirchButton),
            614u32 => Some(ItemKind::JungleButton),
            615u32 => Some(ItemKind::AcaciaButton),
            616u32 => Some(ItemKind::DarkOakButton),
            617u32 => Some(ItemKind::CrimsonButton),
            618u32 => Some(ItemKind::WarpedButton),
            619u32 => Some(ItemKind::StonePressurePlate),
            620u32 => Some(ItemKind::PolishedBlackstonePressurePlate),
            621u32 => Some(ItemKind::LightWeightedPressurePlate),
            622u32 => Some(ItemKind::HeavyWeightedPressurePlate),
            623u32 => Some(ItemKind::OakPressurePlate),
            624u32 => Some(ItemKind::SprucePressurePlate),
            625u32 => Some(ItemKind::BirchPressurePlate),
            626u32 => Some(ItemKind::JunglePressurePlate),
            627u32 => Some(ItemKind::AcaciaPressurePlate),
            628u32 => Some(ItemKind::DarkOakPressurePlate),
            629u32 => Some(ItemKind::CrimsonPressurePlate),
            630u32 => Some(ItemKind::WarpedPressurePlate),
            631u32 => Some(ItemKind::IronDoor),
            632u32 => Some(ItemKind::OakDoor),
            633u32 => Some(ItemKind::SpruceDoor),
            634u32 => Some(ItemKind::BirchDoor),
            635u32 => Some(ItemKind::JungleDoor),
            636u32 => Some(ItemKind::AcaciaDoor),
            637u32 => Some(ItemKind::DarkOakDoor),
            638u32 => Some(ItemKind::CrimsonDoor),
            639u32 => Some(ItemKind::WarpedDoor),
            640u32 => Some(ItemKind::IronTrapdoor),
            641u32 => Some(ItemKind::OakTrapdoor),
            642u32 => Some(ItemKind::SpruceTrapdoor),
            643u32 => Some(ItemKind::BirchTrapdoor),
            644u32 => Some(ItemKind::JungleTrapdoor),
            645u32 => Some(ItemKind::AcaciaTrapdoor),
            646u32 => Some(ItemKind::DarkOakTrapdoor),
            647u32 => Some(ItemKind::CrimsonTrapdoor),
            648u32 => Some(ItemKind::WarpedTrapdoor),
            649u32 => Some(ItemKind::OakFenceGate),
            650u32 => Some(ItemKind::SpruceFenceGate),
            651u32 => Some(ItemKind::BirchFenceGate),
            652u32 => Some(ItemKind::JungleFenceGate),
            653u32 => Some(ItemKind::AcaciaFenceGate),
            654u32 => Some(ItemKind::DarkOakFenceGate),
            655u32 => Some(ItemKind::CrimsonFenceGate),
            656u32 => Some(ItemKind::WarpedFenceGate),
            657u32 => Some(ItemKind::PoweredRail),
            658u32 => Some(ItemKind::DetectorRail),
            659u32 => Some(ItemKind::Rail),
            660u32 => Some(ItemKind::ActivatorRail),
            661u32 => Some(ItemKind::Saddle),
            662u32 => Some(ItemKind::Minecart),
            663u32 => Some(ItemKind::ChestMinecart),
            664u32 => Some(ItemKind::FurnaceMinecart),
            665u32 => Some(ItemKind::TntMinecart),
            666u32 => Some(ItemKind::HopperMinecart),
            667u32 => Some(ItemKind::CarrotOnAStick),
            668u32 => Some(ItemKind::WarpedFungusOnAStick),
            669u32 => Some(ItemKind::Elytra),
            670u32 => Some(ItemKind::OakBoat),
            671u32 => Some(ItemKind::SpruceBoat),
            672u32 => Some(ItemKind::BirchBoat),
            673u32 => Some(ItemKind::JungleBoat),
            674u32 => Some(ItemKind::AcaciaBoat),
            675u32 => Some(ItemKind::DarkOakBoat),
            676u32 => Some(ItemKind::StructureBlock),
            677u32 => Some(ItemKind::Jigsaw),
            678u32 => Some(ItemKind::TurtleHelmet),
            679u32 => Some(ItemKind::Scute),
            680u32 => Some(ItemKind::FlintAndSteel),
            681u32 => Some(ItemKind::Apple),
            682u32 => Some(ItemKind::Bow),
            683u32 => Some(ItemKind::Arrow),
            684u32 => Some(ItemKind::Coal),
            685u32 => Some(ItemKind::Charcoal),
            686u32 => Some(ItemKind::Diamond),
            687u32 => Some(ItemKind::Emerald),
            688u32 => Some(ItemKind::LapisLazuli),
            689u32 => Some(ItemKind::Quartz),
            690u32 => Some(ItemKind::AmethystShard),
            691u32 => Some(ItemKind::RawIron),
            692u32 => Some(ItemKind::IronIngot),
            693u32 => Some(ItemKind::RawCopper),
            694u32 => Some(ItemKind::CopperIngot),
            695u32 => Some(ItemKind::RawGold),
            696u32 => Some(ItemKind::GoldIngot),
            697u32 => Some(ItemKind::NetheriteIngot),
            698u32 => Some(ItemKind::NetheriteScrap),
            699u32 => Some(ItemKind::WoodenSword),
            700u32 => Some(ItemKind::WoodenShovel),
            701u32 => Some(ItemKind::WoodenPickaxe),
            702u32 => Some(ItemKind::WoodenAxe),
            703u32 => Some(ItemKind::WoodenHoe),
            704u32 => Some(ItemKind::StoneSword),
            705u32 => Some(ItemKind::StoneShovel),
            706u32 => Some(ItemKind::StonePickaxe),
            707u32 => Some(ItemKind::StoneAxe),
            708u32 => Some(ItemKind::StoneHoe),
            709u32 => Some(ItemKind::GoldenSword),
            710u32 => Some(ItemKind::GoldenShovel),
            711u32 => Some(ItemKind::GoldenPickaxe),
            712u32 => Some(ItemKind::GoldenAxe),
            713u32 => Some(ItemKind::GoldenHoe),
            714u32 => Some(ItemKind::IronSword),
            715u32 => Some(ItemKind::IronShovel),
            716u32 => Some(ItemKind::IronPickaxe),
            717u32 => Some(ItemKind::IronAxe),
            718u32 => Some(ItemKind::IronHoe),
            719u32 => Some(ItemKind::DiamondSword),
            720u32 => Some(ItemKind::DiamondShovel),
            721u32 => Some(ItemKind::DiamondPickaxe),
            722u32 => Some(ItemKind::DiamondAxe),
            723u32 => Some(ItemKind::DiamondHoe),
            724u32 => Some(ItemKind::NetheriteSword),
            725u32 => Some(ItemKind::NetheriteShovel),
            726u32 => Some(ItemKind::NetheritePickaxe),
            727u32 => Some(ItemKind::NetheriteAxe),
            728u32 => Some(ItemKind::NetheriteHoe),
            729u32 => Some(ItemKind::Stick),
            730u32 => Some(ItemKind::Bowl),
            731u32 => Some(ItemKind::MushroomStew),
            732u32 => Some(ItemKind::String),
            733u32 => Some(ItemKind::Feather),
            734u32 => Some(ItemKind::Gunpowder),
            735u32 => Some(ItemKind::WheatSeeds),
            736u32 => Some(ItemKind::Wheat),
            737u32 => Some(ItemKind::Bread),
            738u32 => Some(ItemKind::LeatherHelmet),
            739u32 => Some(ItemKind::LeatherChestplate),
            740u32 => Some(ItemKind::LeatherLeggings),
            741u32 => Some(ItemKind::LeatherBoots),
            742u32 => Some(ItemKind::ChainmailHelmet),
            743u32 => Some(ItemKind::ChainmailChestplate),
            744u32 => Some(ItemKind::ChainmailLeggings),
            745u32 => Some(ItemKind::ChainmailBoots),
            746u32 => Some(ItemKind::IronHelmet),
            747u32 => Some(ItemKind::IronChestplate),
            748u32 => Some(ItemKind::IronLeggings),
            749u32 => Some(ItemKind::IronBoots),
            750u32 => Some(ItemKind::DiamondHelmet),
            751u32 => Some(ItemKind::DiamondChestplate),
            752u32 => Some(ItemKind::DiamondLeggings),
            753u32 => Some(ItemKind::DiamondBoots),
            754u32 => Some(ItemKind::GoldenHelmet),
            755u32 => Some(ItemKind::GoldenChestplate),
            756u32 => Some(ItemKind::GoldenLeggings),
            757u32 => Some(ItemKind::GoldenBoots),
            758u32 => Some(ItemKind::NetheriteHelmet),
            759u32 => Some(ItemKind::NetheriteChestplate),
            760u32 => Some(ItemKind::NetheriteLeggings),
            761u32 => Some(ItemKind::NetheriteBoots),
            762u32 => Some(ItemKind::Flint),
            763u32 => Some(ItemKind::Porkchop),
            764u32 => Some(ItemKind::CookedPorkchop),
            765u32 => Some(ItemKind::Painting),
            766u32 => Some(ItemKind::GoldenApple),
            767u32 => Some(ItemKind::EnchantedGoldenApple),
            768u32 => Some(ItemKind::OakSign),
            769u32 => Some(ItemKind::SpruceSign),
            770u32 => Some(ItemKind::BirchSign),
            771u32 => Some(ItemKind::JungleSign),
            772u32 => Some(ItemKind::AcaciaSign),
            773u32 => Some(ItemKind::DarkOakSign),
            774u32 => Some(ItemKind::CrimsonSign),
            775u32 => Some(ItemKind::WarpedSign),
            776u32 => Some(ItemKind::Bucket),
            777u32 => Some(ItemKind::WaterBucket),
            778u32 => Some(ItemKind::LavaBucket),
            779u32 => Some(ItemKind::PowderSnowBucket),
            780u32 => Some(ItemKind::Snowball),
            781u32 => Some(ItemKind::Leather),
            782u32 => Some(ItemKind::MilkBucket),
            783u32 => Some(ItemKind::PufferfishBucket),
            784u32 => Some(ItemKind::SalmonBucket),
            785u32 => Some(ItemKind::CodBucket),
            786u32 => Some(ItemKind::TropicalFishBucket),
            787u32 => Some(ItemKind::AxolotlBucket),
            788u32 => Some(ItemKind::Brick),
            789u32 => Some(ItemKind::ClayBall),
            790u32 => Some(ItemKind::DriedKelpBlock),
            791u32 => Some(ItemKind::Paper),
            792u32 => Some(ItemKind::Book),
            793u32 => Some(ItemKind::SlimeBall),
            794u32 => Some(ItemKind::Egg),
            795u32 => Some(ItemKind::Compass),
            796u32 => Some(ItemKind::Bundle),
            797u32 => Some(ItemKind::FishingRod),
            798u32 => Some(ItemKind::Clock),
            799u32 => Some(ItemKind::Spyglass),
            800u32 => Some(ItemKind::GlowstoneDust),
            801u32 => Some(ItemKind::Cod),
            802u32 => Some(ItemKind::Salmon),
            803u32 => Some(ItemKind::TropicalFish),
            804u32 => Some(ItemKind::Pufferfish),
            805u32 => Some(ItemKind::CookedCod),
            806u32 => Some(ItemKind::CookedSalmon),
            807u32 => Some(ItemKind::InkSac),
            808u32 => Some(ItemKind::GlowInkSac),
            809u32 => Some(ItemKind::CocoaBeans),
            810u32 => Some(ItemKind::WhiteDye),
            811u32 => Some(ItemKind::OrangeDye),
            812u32 => Some(ItemKind::MagentaDye),
            813u32 => Some(ItemKind::LightBlueDye),
            814u32 => Some(ItemKind::YellowDye),
            815u32 => Some(ItemKind::LimeDye),
            816u32 => Some(ItemKind::PinkDye),
            817u32 => Some(ItemKind::GrayDye),
            818u32 => Some(ItemKind::LightGrayDye),
            819u32 => Some(ItemKind::CyanDye),
            820u32 => Some(ItemKind::PurpleDye),
            821u32 => Some(ItemKind::BlueDye),
            822u32 => Some(ItemKind::BrownDye),
            823u32 => Some(ItemKind::GreenDye),
            824u32 => Some(ItemKind::RedDye),
            825u32 => Some(ItemKind::BlackDye),
            826u32 => Some(ItemKind::BoneMeal),
            827u32 => Some(ItemKind::Bone),
            828u32 => Some(ItemKind::Sugar),
            829u32 => Some(ItemKind::Cake),
            830u32 => Some(ItemKind::WhiteBed),
            831u32 => Some(ItemKind::OrangeBed),
            832u32 => Some(ItemKind::MagentaBed),
            833u32 => Some(ItemKind::LightBlueBed),
            834u32 => Some(ItemKind::YellowBed),
            835u32 => Some(ItemKind::LimeBed),
            836u32 => Some(ItemKind::PinkBed),
            837u32 => Some(ItemKind::GrayBed),
            838u32 => Some(ItemKind::LightGrayBed),
            839u32 => Some(ItemKind::CyanBed),
            840u32 => Some(ItemKind::PurpleBed),
            841u32 => Some(ItemKind::BlueBed),
            842u32 => Some(ItemKind::BrownBed),
            843u32 => Some(ItemKind::GreenBed),
            844u32 => Some(ItemKind::RedBed),
            845u32 => Some(ItemKind::BlackBed),
            846u32 => Some(ItemKind::Cookie),
            847u32 => Some(ItemKind::FilledMap),
            848u32 => Some(ItemKind::Shears),
            849u32 => Some(ItemKind::MelonSlice),
            850u32 => Some(ItemKind::DriedKelp),
            851u32 => Some(ItemKind::PumpkinSeeds),
            852u32 => Some(ItemKind::MelonSeeds),
            853u32 => Some(ItemKind::Beef),
            854u32 => Some(ItemKind::CookedBeef),
            855u32 => Some(ItemKind::Chicken),
            856u32 => Some(ItemKind::CookedChicken),
            857u32 => Some(ItemKind::RottenFlesh),
            858u32 => Some(ItemKind::EnderPearl),
            859u32 => Some(ItemKind::BlazeRod),
            860u32 => Some(ItemKind::GhastTear),
            861u32 => Some(ItemKind::GoldNugget),
            862u32 => Some(ItemKind::NetherWart),
            863u32 => Some(ItemKind::Potion),
            864u32 => Some(ItemKind::GlassBottle),
            865u32 => Some(ItemKind::SpiderEye),
            866u32 => Some(ItemKind::FermentedSpiderEye),
            867u32 => Some(ItemKind::BlazePowder),
            868u32 => Some(ItemKind::MagmaCream),
            869u32 => Some(ItemKind::BrewingStand),
            870u32 => Some(ItemKind::Cauldron),
            871u32 => Some(ItemKind::EnderEye),
            872u32 => Some(ItemKind::GlisteringMelonSlice),
            873u32 => Some(ItemKind::AxolotlSpawnEgg),
            874u32 => Some(ItemKind::BatSpawnEgg),
            875u32 => Some(ItemKind::BeeSpawnEgg),
            876u32 => Some(ItemKind::BlazeSpawnEgg),
            877u32 => Some(ItemKind::CatSpawnEgg),
            878u32 => Some(ItemKind::CaveSpiderSpawnEgg),
            879u32 => Some(ItemKind::ChickenSpawnEgg),
            880u32 => Some(ItemKind::CodSpawnEgg),
            881u32 => Some(ItemKind::CowSpawnEgg),
            882u32 => Some(ItemKind::CreeperSpawnEgg),
            883u32 => Some(ItemKind::DolphinSpawnEgg),
            884u32 => Some(ItemKind::DonkeySpawnEgg),
            885u32 => Some(ItemKind::DrownedSpawnEgg),
            886u32 => Some(ItemKind::ElderGuardianSpawnEgg),
            887u32 => Some(ItemKind::EndermanSpawnEgg),
            888u32 => Some(ItemKind::EndermiteSpawnEgg),
            889u32 => Some(ItemKind::EvokerSpawnEgg),
            890u32 => Some(ItemKind::FoxSpawnEgg),
            891u32 => Some(ItemKind::GhastSpawnEgg),
            892u32 => Some(ItemKind::GlowSquidSpawnEgg),
            893u32 => Some(ItemKind::GoatSpawnEgg),
            894u32 => Some(ItemKind::GuardianSpawnEgg),
            895u32 => Some(ItemKind::HoglinSpawnEgg),
            896u32 => Some(ItemKind::HorseSpawnEgg),
            897u32 => Some(ItemKind::HuskSpawnEgg),
            898u32 => Some(ItemKind::LlamaSpawnEgg),
            899u32 => Some(ItemKind::MagmaCubeSpawnEgg),
            900u32 => Some(ItemKind::MooshroomSpawnEgg),
            901u32 => Some(ItemKind::MuleSpawnEgg),
            902u32 => Some(ItemKind::OcelotSpawnEgg),
            903u32 => Some(ItemKind::PandaSpawnEgg),
            904u32 => Some(ItemKind::ParrotSpawnEgg),
            905u32 => Some(ItemKind::PhantomSpawnEgg),
            906u32 => Some(ItemKind::PigSpawnEgg),
            907u32 => Some(ItemKind::PiglinSpawnEgg),
            908u32 => Some(ItemKind::PiglinBruteSpawnEgg),
            909u32 => Some(ItemKind::PillagerSpawnEgg),
            910u32 => Some(ItemKind::PolarBearSpawnEgg),
            911u32 => Some(ItemKind::PufferfishSpawnEgg),
            912u32 => Some(ItemKind::RabbitSpawnEgg),
            913u32 => Some(ItemKind::RavagerSpawnEgg),
            914u32 => Some(ItemKind::SalmonSpawnEgg),
            915u32 => Some(ItemKind::SheepSpawnEgg),
            916u32 => Some(ItemKind::ShulkerSpawnEgg),
            917u32 => Some(ItemKind::SilverfishSpawnEgg),
            918u32 => Some(ItemKind::SkeletonSpawnEgg),
            919u32 => Some(ItemKind::SkeletonHorseSpawnEgg),
            920u32 => Some(ItemKind::SlimeSpawnEgg),
            921u32 => Some(ItemKind::SpiderSpawnEgg),
            922u32 => Some(ItemKind::SquidSpawnEgg),
            923u32 => Some(ItemKind::StraySpawnEgg),
            924u32 => Some(ItemKind::StriderSpawnEgg),
            925u32 => Some(ItemKind::TraderLlamaSpawnEgg),
            926u32 => Some(ItemKind::TropicalFishSpawnEgg),
            927u32 => Some(ItemKind::TurtleSpawnEgg),
            928u32 => Some(ItemKind::VexSpawnEgg),
            929u32 => Some(ItemKind::VillagerSpawnEgg),
            930u32 => Some(ItemKind::VindicatorSpawnEgg),
            931u32 => Some(ItemKind::WanderingTraderSpawnEgg),
            932u32 => Some(ItemKind::WitchSpawnEgg),
            933u32 => Some(ItemKind::WitherSkeletonSpawnEgg),
            934u32 => Some(ItemKind::WolfSpawnEgg),
            935u32 => Some(ItemKind::ZoglinSpawnEgg),
            936u32 => Some(ItemKind::ZombieSpawnEgg),
            937u32 => Some(ItemKind::ZombieHorseSpawnEgg),
            938u32 => Some(ItemKind::ZombieVillagerSpawnEgg),
            939u32 => Some(ItemKind::ZombifiedPiglinSpawnEgg),
            940u32 => Some(ItemKind::ExperienceBottle),
            941u32 => Some(ItemKind::FireCharge),
            942u32 => Some(ItemKind::WritableBook),
            943u32 => Some(ItemKind::WrittenBook),
            944u32 => Some(ItemKind::ItemFrame),
            945u32 => Some(ItemKind::GlowItemFrame),
            946u32 => Some(ItemKind::FlowerPot),
            947u32 => Some(ItemKind::Carrot),
            948u32 => Some(ItemKind::Potato),
            949u32 => Some(ItemKind::BakedPotato),
            950u32 => Some(ItemKind::PoisonousPotato),
            951u32 => Some(ItemKind::Map),
            952u32 => Some(ItemKind::GoldenCarrot),
            953u32 => Some(ItemKind::SkeletonSkull),
            954u32 => Some(ItemKind::WitherSkeletonSkull),
            955u32 => Some(ItemKind::PlayerHead),
            956u32 => Some(ItemKind::ZombieHead),
            957u32 => Some(ItemKind::CreeperHead),
            958u32 => Some(ItemKind::DragonHead),
            959u32 => Some(ItemKind::NetherStar),
            960u32 => Some(ItemKind::PumpkinPie),
            961u32 => Some(ItemKind::FireworkRocket),
            962u32 => Some(ItemKind::FireworkStar),
            963u32 => Some(ItemKind::EnchantedBook),
            964u32 => Some(ItemKind::NetherBrick),
            965u32 => Some(ItemKind::PrismarineShard),
            966u32 => Some(ItemKind::PrismarineCrystals),
            967u32 => Some(ItemKind::Rabbit),
            968u32 => Some(ItemKind::CookedRabbit),
            969u32 => Some(ItemKind::RabbitStew),
            970u32 => Some(ItemKind::RabbitFoot),
            971u32 => Some(ItemKind::RabbitHide),
            972u32 => Some(ItemKind::ArmorStand),
            973u32 => Some(ItemKind::IronHorseArmor),
            974u32 => Some(ItemKind::GoldenHorseArmor),
            975u32 => Some(ItemKind::DiamondHorseArmor),
            976u32 => Some(ItemKind::LeatherHorseArmor),
            977u32 => Some(ItemKind::Lead),
            978u32 => Some(ItemKind::NameTag),
            979u32 => Some(ItemKind::CommandBlockMinecart),
            980u32 => Some(ItemKind::Mutton),
            981u32 => Some(ItemKind::CookedMutton),
            982u32 => Some(ItemKind::WhiteBanner),
            983u32 => Some(ItemKind::OrangeBanner),
            984u32 => Some(ItemKind::MagentaBanner),
            985u32 => Some(ItemKind::LightBlueBanner),
            986u32 => Some(ItemKind::YellowBanner),
            987u32 => Some(ItemKind::LimeBanner),
            988u32 => Some(ItemKind::PinkBanner),
            989u32 => Some(ItemKind::GrayBanner),
            990u32 => Some(ItemKind::LightGrayBanner),
            991u32 => Some(ItemKind::CyanBanner),
            992u32 => Some(ItemKind::PurpleBanner),
            993u32 => Some(ItemKind::BlueBanner),
            994u32 => Some(ItemKind::BrownBanner),
            995u32 => Some(ItemKind::GreenBanner),
            996u32 => Some(ItemKind::RedBanner),
            997u32 => Some(ItemKind::BlackBanner),
            998u32 => Some(ItemKind::EndCrystal),
            999u32 => Some(ItemKind::ChorusFruit),
            1000u32 => Some(ItemKind::PoppedChorusFruit),
            1001u32 => Some(ItemKind::Beetroot),
            1002u32 => Some(ItemKind::BeetrootSeeds),
            1003u32 => Some(ItemKind::BeetrootSoup),
            1004u32 => Some(ItemKind::DragonBreath),
            1005u32 => Some(ItemKind::SplashPotion),
            1006u32 => Some(ItemKind::SpectralArrow),
            1007u32 => Some(ItemKind::TippedArrow),
            1008u32 => Some(ItemKind::LingeringPotion),
            1009u32 => Some(ItemKind::Shield),
            1010u32 => Some(ItemKind::TotemOfUndying),
            1011u32 => Some(ItemKind::ShulkerShell),
            1012u32 => Some(ItemKind::IronNugget),
            1013u32 => Some(ItemKind::KnowledgeBook),
            1014u32 => Some(ItemKind::DebugStick),
            1015u32 => Some(ItemKind::MusicDisc13),
            1016u32 => Some(ItemKind::MusicDiscCat),
            1017u32 => Some(ItemKind::MusicDiscBlocks),
            1018u32 => Some(ItemKind::MusicDiscChirp),
            1019u32 => Some(ItemKind::MusicDiscFar),
            1020u32 => Some(ItemKind::MusicDiscMall),
            1021u32 => Some(ItemKind::MusicDiscMellohi),
            1022u32 => Some(ItemKind::MusicDiscStal),
            1023u32 => Some(ItemKind::MusicDiscStrad),
            1024u32 => Some(ItemKind::MusicDiscWard),
            1025u32 => Some(ItemKind::MusicDisc11),
            1026u32 => Some(ItemKind::MusicDiscWait),
            1027u32 => Some(ItemKind::MusicDiscOtherside),
            1028u32 => Some(ItemKind::MusicDiscPigstep),
            1029u32 => Some(ItemKind::Trident),
            1030u32 => Some(ItemKind::PhantomMembrane),
            1031u32 => Some(ItemKind::NautilusShell),
            1032u32 => Some(ItemKind::HeartOfTheSea),
            1033u32 => Some(ItemKind::Crossbow),
            1034u32 => Some(ItemKind::SuspiciousStew),
            1035u32 => Some(ItemKind::Loom),
            1036u32 => Some(ItemKind::FlowerBannerPattern),
            1037u32 => Some(ItemKind::CreeperBannerPattern),
            1038u32 => Some(ItemKind::SkullBannerPattern),
            1039u32 => Some(ItemKind::MojangBannerPattern),
            1040u32 => Some(ItemKind::GlobeBannerPattern),
            1041u32 => Some(ItemKind::PiglinBannerPattern),
            1042u32 => Some(ItemKind::Composter),
            1043u32 => Some(ItemKind::Barrel),
            1044u32 => Some(ItemKind::Smoker),
            1045u32 => Some(ItemKind::BlastFurnace),
            1046u32 => Some(ItemKind::CartographyTable),
            1047u32 => Some(ItemKind::FletchingTable),
            1048u32 => Some(ItemKind::Grindstone),
            1049u32 => Some(ItemKind::SmithingTable),
            1050u32 => Some(ItemKind::Stonecutter),
            1051u32 => Some(ItemKind::Bell),
            1052u32 => Some(ItemKind::Lantern),
            1053u32 => Some(ItemKind::SoulLantern),
            1054u32 => Some(ItemKind::SweetBerries),
            1055u32 => Some(ItemKind::GlowBerries),
            1056u32 => Some(ItemKind::Campfire),
            1057u32 => Some(ItemKind::SoulCampfire),
            1058u32 => Some(ItemKind::Shroomlight),
            1059u32 => Some(ItemKind::Honeycomb),
            1060u32 => Some(ItemKind::BeeNest),
            1061u32 => Some(ItemKind::Beehive),
            1062u32 => Some(ItemKind::HoneyBottle),
            1063u32 => Some(ItemKind::HoneycombBlock),
            1064u32 => Some(ItemKind::Lodestone),
            1065u32 => Some(ItemKind::CryingObsidian),
            1066u32 => Some(ItemKind::Blackstone),
            1067u32 => Some(ItemKind::BlackstoneSlab),
            1068u32 => Some(ItemKind::BlackstoneStairs),
            1069u32 => Some(ItemKind::GildedBlackstone),
            1070u32 => Some(ItemKind::PolishedBlackstone),
            1071u32 => Some(ItemKind::PolishedBlackstoneSlab),
            1072u32 => Some(ItemKind::PolishedBlackstoneStairs),
            1073u32 => Some(ItemKind::ChiseledPolishedBlackstone),
            1074u32 => Some(ItemKind::PolishedBlackstoneBricks),
            1075u32 => Some(ItemKind::PolishedBlackstoneBrickSlab),
            1076u32 => Some(ItemKind::PolishedBlackstoneBrickStairs),
            1077u32 => Some(ItemKind::CrackedPolishedBlackstoneBricks),
            1078u32 => Some(ItemKind::RespawnAnchor),
            1079u32 => Some(ItemKind::Candle),
            1080u32 => Some(ItemKind::WhiteCandle),
            1081u32 => Some(ItemKind::OrangeCandle),
            1082u32 => Some(ItemKind::MagentaCandle),
            1083u32 => Some(ItemKind::LightBlueCandle),
            1084u32 => Some(ItemKind::YellowCandle),
            1085u32 => Some(ItemKind::LimeCandle),
            1086u32 => Some(ItemKind::PinkCandle),
            1087u32 => Some(ItemKind::GrayCandle),
            1088u32 => Some(ItemKind::LightGrayCandle),
            1089u32 => Some(ItemKind::CyanCandle),
            1090u32 => Some(ItemKind::PurpleCandle),
            1091u32 => Some(ItemKind::BlueCandle),
            1092u32 => Some(ItemKind::BrownCandle),
            1093u32 => Some(ItemKind::GreenCandle),
            1094u32 => Some(ItemKind::RedCandle),
            1095u32 => Some(ItemKind::BlackCandle),
            1096u32 => Some(ItemKind::SmallAmethystBud),
            1097u32 => Some(ItemKind::MediumAmethystBud),
            1098u32 => Some(ItemKind::LargeAmethystBud),
            1099u32 => Some(ItemKind::AmethystCluster),
            1100u32 => Some(ItemKind::PointedDripstone),
            _ => None,
        }
    }
}
impl ItemKind {
    #[doc = "Returns the `name` property of this `ItemKind`."]
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            ItemKind::Stone => "stone",
            ItemKind::Granite => "granite",
            ItemKind::PolishedGranite => "polished_granite",
            ItemKind::Diorite => "diorite",
            ItemKind::PolishedDiorite => "polished_diorite",
            ItemKind::Andesite => "andesite",
            ItemKind::PolishedAndesite => "polished_andesite",
            ItemKind::Deepslate => "deepslate",
            ItemKind::CobbledDeepslate => "cobbled_deepslate",
            ItemKind::PolishedDeepslate => "polished_deepslate",
            ItemKind::Calcite => "calcite",
            ItemKind::Tuff => "tuff",
            ItemKind::DripstoneBlock => "dripstone_block",
            ItemKind::GrassBlock => "grass_block",
            ItemKind::Dirt => "dirt",
            ItemKind::CoarseDirt => "coarse_dirt",
            ItemKind::Podzol => "podzol",
            ItemKind::RootedDirt => "rooted_dirt",
            ItemKind::CrimsonNylium => "crimson_nylium",
            ItemKind::WarpedNylium => "warped_nylium",
            ItemKind::Cobblestone => "cobblestone",
            ItemKind::OakPlanks => "oak_planks",
            ItemKind::SprucePlanks => "spruce_planks",
            ItemKind::BirchPlanks => "birch_planks",
            ItemKind::JunglePlanks => "jungle_planks",
            ItemKind::AcaciaPlanks => "acacia_planks",
            ItemKind::DarkOakPlanks => "dark_oak_planks",
            ItemKind::CrimsonPlanks => "crimson_planks",
            ItemKind::WarpedPlanks => "warped_planks",
            ItemKind::OakSapling => "oak_sapling",
            ItemKind::SpruceSapling => "spruce_sapling",
            ItemKind::BirchSapling => "birch_sapling",
            ItemKind::JungleSapling => "jungle_sapling",
            ItemKind::AcaciaSapling => "acacia_sapling",
            ItemKind::DarkOakSapling => "dark_oak_sapling",
            ItemKind::Bedrock => "bedrock",
            ItemKind::Sand => "sand",
            ItemKind::RedSand => "red_sand",
            ItemKind::Gravel => "gravel",
            ItemKind::CoalOre => "coal_ore",
            ItemKind::DeepslateCoalOre => "deepslate_coal_ore",
            ItemKind::IronOre => "iron_ore",
            ItemKind::DeepslateIronOre => "deepslate_iron_ore",
            ItemKind::CopperOre => "copper_ore",
            ItemKind::DeepslateCopperOre => "deepslate_copper_ore",
            ItemKind::GoldOre => "gold_ore",
            ItemKind::DeepslateGoldOre => "deepslate_gold_ore",
            ItemKind::RedstoneOre => "redstone_ore",
            ItemKind::DeepslateRedstoneOre => "deepslate_redstone_ore",
            ItemKind::EmeraldOre => "emerald_ore",
            ItemKind::DeepslateEmeraldOre => "deepslate_emerald_ore",
            ItemKind::LapisOre => "lapis_ore",
            ItemKind::DeepslateLapisOre => "deepslate_lapis_ore",
            ItemKind::DiamondOre => "diamond_ore",
            ItemKind::DeepslateDiamondOre => "deepslate_diamond_ore",
            ItemKind::NetherGoldOre => "nether_gold_ore",
            ItemKind::NetherQuartzOre => "nether_quartz_ore",
            ItemKind::AncientDebris => "ancient_debris",
            ItemKind::CoalBlock => "coal_block",
            ItemKind::RawIronBlock => "raw_iron_block",
            ItemKind::RawCopperBlock => "raw_copper_block",
            ItemKind::RawGoldBlock => "raw_gold_block",
            ItemKind::AmethystBlock => "amethyst_block",
            ItemKind::BuddingAmethyst => "budding_amethyst",
            ItemKind::IronBlock => "iron_block",
            ItemKind::CopperBlock => "copper_block",
            ItemKind::GoldBlock => "gold_block",
            ItemKind::DiamondBlock => "diamond_block",
            ItemKind::NetheriteBlock => "netherite_block",
            ItemKind::ExposedCopper => "exposed_copper",
            ItemKind::WeatheredCopper => "weathered_copper",
            ItemKind::OxidizedCopper => "oxidized_copper",
            ItemKind::CutCopper => "cut_copper",
            ItemKind::ExposedCutCopper => "exposed_cut_copper",
            ItemKind::WeatheredCutCopper => "weathered_cut_copper",
            ItemKind::OxidizedCutCopper => "oxidized_cut_copper",
            ItemKind::CutCopperStairs => "cut_copper_stairs",
            ItemKind::ExposedCutCopperStairs => "exposed_cut_copper_stairs",
            ItemKind::WeatheredCutCopperStairs => "weathered_cut_copper_stairs",
            ItemKind::OxidizedCutCopperStairs => "oxidized_cut_copper_stairs",
            ItemKind::CutCopperSlab => "cut_copper_slab",
            ItemKind::ExposedCutCopperSlab => "exposed_cut_copper_slab",
            ItemKind::WeatheredCutCopperSlab => "weathered_cut_copper_slab",
            ItemKind::OxidizedCutCopperSlab => "oxidized_cut_copper_slab",
            ItemKind::WaxedCopperBlock => "waxed_copper_block",
            ItemKind::WaxedExposedCopper => "waxed_exposed_copper",
            ItemKind::WaxedWeatheredCopper => "waxed_weathered_copper",
            ItemKind::WaxedOxidizedCopper => "waxed_oxidized_copper",
            ItemKind::WaxedCutCopper => "waxed_cut_copper",
            ItemKind::WaxedExposedCutCopper => "waxed_exposed_cut_copper",
            ItemKind::WaxedWeatheredCutCopper => "waxed_weathered_cut_copper",
            ItemKind::WaxedOxidizedCutCopper => "waxed_oxidized_cut_copper",
            ItemKind::WaxedCutCopperStairs => "waxed_cut_copper_stairs",
            ItemKind::WaxedExposedCutCopperStairs => "waxed_exposed_cut_copper_stairs",
            ItemKind::WaxedWeatheredCutCopperStairs => "waxed_weathered_cut_copper_stairs",
            ItemKind::WaxedOxidizedCutCopperStairs => "waxed_oxidized_cut_copper_stairs",
            ItemKind::WaxedCutCopperSlab => "waxed_cut_copper_slab",
            ItemKind::WaxedExposedCutCopperSlab => "waxed_exposed_cut_copper_slab",
            ItemKind::WaxedWeatheredCutCopperSlab => "waxed_weathered_cut_copper_slab",
            ItemKind::WaxedOxidizedCutCopperSlab => "waxed_oxidized_cut_copper_slab",
            ItemKind::OakLog => "oak_log",
            ItemKind::SpruceLog => "spruce_log",
            ItemKind::BirchLog => "birch_log",
            ItemKind::JungleLog => "jungle_log",
            ItemKind::AcaciaLog => "acacia_log",
            ItemKind::DarkOakLog => "dark_oak_log",
            ItemKind::CrimsonStem => "crimson_stem",
            ItemKind::WarpedStem => "warped_stem",
            ItemKind::StrippedOakLog => "stripped_oak_log",
            ItemKind::StrippedSpruceLog => "stripped_spruce_log",
            ItemKind::StrippedBirchLog => "stripped_birch_log",
            ItemKind::StrippedJungleLog => "stripped_jungle_log",
            ItemKind::StrippedAcaciaLog => "stripped_acacia_log",
            ItemKind::StrippedDarkOakLog => "stripped_dark_oak_log",
            ItemKind::StrippedCrimsonStem => "stripped_crimson_stem",
            ItemKind::StrippedWarpedStem => "stripped_warped_stem",
            ItemKind::StrippedOakWood => "stripped_oak_wood",
            ItemKind::StrippedSpruceWood => "stripped_spruce_wood",
            ItemKind::StrippedBirchWood => "stripped_birch_wood",
            ItemKind::StrippedJungleWood => "stripped_jungle_wood",
            ItemKind::StrippedAcaciaWood => "stripped_acacia_wood",
            ItemKind::StrippedDarkOakWood => "stripped_dark_oak_wood",
            ItemKind::StrippedCrimsonHyphae => "stripped_crimson_hyphae",
            ItemKind::StrippedWarpedHyphae => "stripped_warped_hyphae",
            ItemKind::OakWood => "oak_wood",
            ItemKind::SpruceWood => "spruce_wood",
            ItemKind::BirchWood => "birch_wood",
            ItemKind::JungleWood => "jungle_wood",
            ItemKind::AcaciaWood => "acacia_wood",
            ItemKind::DarkOakWood => "dark_oak_wood",
            ItemKind::CrimsonHyphae => "crimson_hyphae",
            ItemKind::WarpedHyphae => "warped_hyphae",
            ItemKind::OakLeaves => "oak_leaves",
            ItemKind::SpruceLeaves => "spruce_leaves",
            ItemKind::BirchLeaves => "birch_leaves",
            ItemKind::JungleLeaves => "jungle_leaves",
            ItemKind::AcaciaLeaves => "acacia_leaves",
            ItemKind::DarkOakLeaves => "dark_oak_leaves",
            ItemKind::AzaleaLeaves => "azalea_leaves",
            ItemKind::FloweringAzaleaLeaves => "flowering_azalea_leaves",
            ItemKind::Sponge => "sponge",
            ItemKind::WetSponge => "wet_sponge",
            ItemKind::Glass => "glass",
            ItemKind::TintedGlass => "tinted_glass",
            ItemKind::LapisBlock => "lapis_block",
            ItemKind::Sandstone => "sandstone",
            ItemKind::ChiseledSandstone => "chiseled_sandstone",
            ItemKind::CutSandstone => "cut_sandstone",
            ItemKind::Cobweb => "cobweb",
            ItemKind::Grass => "grass",
            ItemKind::Fern => "fern",
            ItemKind::Azalea => "azalea",
            ItemKind::FloweringAzalea => "flowering_azalea",
            ItemKind::DeadBush => "dead_bush",
            ItemKind::Seagrass => "seagrass",
            ItemKind::SeaPickle => "sea_pickle",
            ItemKind::WhiteWool => "white_wool",
            ItemKind::OrangeWool => "orange_wool",
            ItemKind::MagentaWool => "magenta_wool",
            ItemKind::LightBlueWool => "light_blue_wool",
            ItemKind::YellowWool => "yellow_wool",
            ItemKind::LimeWool => "lime_wool",
            ItemKind::PinkWool => "pink_wool",
            ItemKind::GrayWool => "gray_wool",
            ItemKind::LightGrayWool => "light_gray_wool",
            ItemKind::CyanWool => "cyan_wool",
            ItemKind::PurpleWool => "purple_wool",
            ItemKind::BlueWool => "blue_wool",
            ItemKind::BrownWool => "brown_wool",
            ItemKind::GreenWool => "green_wool",
            ItemKind::RedWool => "red_wool",
            ItemKind::BlackWool => "black_wool",
            ItemKind::Dandelion => "dandelion",
            ItemKind::Poppy => "poppy",
            ItemKind::BlueOrchid => "blue_orchid",
            ItemKind::Allium => "allium",
            ItemKind::AzureBluet => "azure_bluet",
            ItemKind::RedTulip => "red_tulip",
            ItemKind::OrangeTulip => "orange_tulip",
            ItemKind::WhiteTulip => "white_tulip",
            ItemKind::PinkTulip => "pink_tulip",
            ItemKind::OxeyeDaisy => "oxeye_daisy",
            ItemKind::Cornflower => "cornflower",
            ItemKind::LilyOfTheValley => "lily_of_the_valley",
            ItemKind::WitherRose => "wither_rose",
            ItemKind::SporeBlossom => "spore_blossom",
            ItemKind::BrownMushroom => "brown_mushroom",
            ItemKind::RedMushroom => "red_mushroom",
            ItemKind::CrimsonFungus => "crimson_fungus",
            ItemKind::WarpedFungus => "warped_fungus",
            ItemKind::CrimsonRoots => "crimson_roots",
            ItemKind::WarpedRoots => "warped_roots",
            ItemKind::NetherSprouts => "nether_sprouts",
            ItemKind::WeepingVines => "weeping_vines",
            ItemKind::TwistingVines => "twisting_vines",
            ItemKind::SugarCane => "sugar_cane",
            ItemKind::Kelp => "kelp",
            ItemKind::MossCarpet => "moss_carpet",
            ItemKind::MossBlock => "moss_block",
            ItemKind::HangingRoots => "hanging_roots",
            ItemKind::BigDripleaf => "big_dripleaf",
            ItemKind::SmallDripleaf => "small_dripleaf",
            ItemKind::Bamboo => "bamboo",
            ItemKind::OakSlab => "oak_slab",
            ItemKind::SpruceSlab => "spruce_slab",
            ItemKind::BirchSlab => "birch_slab",
            ItemKind::JungleSlab => "jungle_slab",
            ItemKind::AcaciaSlab => "acacia_slab",
            ItemKind::DarkOakSlab => "dark_oak_slab",
            ItemKind::CrimsonSlab => "crimson_slab",
            ItemKind::WarpedSlab => "warped_slab",
            ItemKind::StoneSlab => "stone_slab",
            ItemKind::SmoothStoneSlab => "smooth_stone_slab",
            ItemKind::SandstoneSlab => "sandstone_slab",
            ItemKind::CutSandstoneSlab => "cut_sandstone_slab",
            ItemKind::PetrifiedOakSlab => "petrified_oak_slab",
            ItemKind::CobblestoneSlab => "cobblestone_slab",
            ItemKind::BrickSlab => "brick_slab",
            ItemKind::StoneBrickSlab => "stone_brick_slab",
            ItemKind::NetherBrickSlab => "nether_brick_slab",
            ItemKind::QuartzSlab => "quartz_slab",
            ItemKind::RedSandstoneSlab => "red_sandstone_slab",
            ItemKind::CutRedSandstoneSlab => "cut_red_sandstone_slab",
            ItemKind::PurpurSlab => "purpur_slab",
            ItemKind::PrismarineSlab => "prismarine_slab",
            ItemKind::PrismarineBrickSlab => "prismarine_brick_slab",
            ItemKind::DarkPrismarineSlab => "dark_prismarine_slab",
            ItemKind::SmoothQuartz => "smooth_quartz",
            ItemKind::SmoothRedSandstone => "smooth_red_sandstone",
            ItemKind::SmoothSandstone => "smooth_sandstone",
            ItemKind::SmoothStone => "smooth_stone",
            ItemKind::Bricks => "bricks",
            ItemKind::Bookshelf => "bookshelf",
            ItemKind::MossyCobblestone => "mossy_cobblestone",
            ItemKind::Obsidian => "obsidian",
            ItemKind::Torch => "torch",
            ItemKind::EndRod => "end_rod",
            ItemKind::ChorusPlant => "chorus_plant",
            ItemKind::ChorusFlower => "chorus_flower",
            ItemKind::PurpurBlock => "purpur_block",
            ItemKind::PurpurPillar => "purpur_pillar",
            ItemKind::PurpurStairs => "purpur_stairs",
            ItemKind::Spawner => "spawner",
            ItemKind::OakStairs => "oak_stairs",
            ItemKind::Chest => "chest",
            ItemKind::CraftingTable => "crafting_table",
            ItemKind::Farmland => "farmland",
            ItemKind::Furnace => "furnace",
            ItemKind::Ladder => "ladder",
            ItemKind::CobblestoneStairs => "cobblestone_stairs",
            ItemKind::Snow => "snow",
            ItemKind::Ice => "ice",
            ItemKind::SnowBlock => "snow_block",
            ItemKind::Cactus => "cactus",
            ItemKind::Clay => "clay",
            ItemKind::Jukebox => "jukebox",
            ItemKind::OakFence => "oak_fence",
            ItemKind::SpruceFence => "spruce_fence",
            ItemKind::BirchFence => "birch_fence",
            ItemKind::JungleFence => "jungle_fence",
            ItemKind::AcaciaFence => "acacia_fence",
            ItemKind::DarkOakFence => "dark_oak_fence",
            ItemKind::CrimsonFence => "crimson_fence",
            ItemKind::WarpedFence => "warped_fence",
            ItemKind::Pumpkin => "pumpkin",
            ItemKind::CarvedPumpkin => "carved_pumpkin",
            ItemKind::JackOLantern => "jack_o_lantern",
            ItemKind::Netherrack => "netherrack",
            ItemKind::SoulSand => "soul_sand",
            ItemKind::SoulSoil => "soul_soil",
            ItemKind::Basalt => "basalt",
            ItemKind::PolishedBasalt => "polished_basalt",
            ItemKind::SmoothBasalt => "smooth_basalt",
            ItemKind::SoulTorch => "soul_torch",
            ItemKind::Glowstone => "glowstone",
            ItemKind::InfestedStone => "infested_stone",
            ItemKind::InfestedCobblestone => "infested_cobblestone",
            ItemKind::InfestedStoneBricks => "infested_stone_bricks",
            ItemKind::InfestedMossyStoneBricks => "infested_mossy_stone_bricks",
            ItemKind::InfestedCrackedStoneBricks => "infested_cracked_stone_bricks",
            ItemKind::InfestedChiseledStoneBricks => "infested_chiseled_stone_bricks",
            ItemKind::InfestedDeepslate => "infested_deepslate",
            ItemKind::StoneBricks => "stone_bricks",
            ItemKind::MossyStoneBricks => "mossy_stone_bricks",
            ItemKind::CrackedStoneBricks => "cracked_stone_bricks",
            ItemKind::ChiseledStoneBricks => "chiseled_stone_bricks",
            ItemKind::DeepslateBricks => "deepslate_bricks",
            ItemKind::CrackedDeepslateBricks => "cracked_deepslate_bricks",
            ItemKind::DeepslateTiles => "deepslate_tiles",
            ItemKind::CrackedDeepslateTiles => "cracked_deepslate_tiles",
            ItemKind::ChiseledDeepslate => "chiseled_deepslate",
            ItemKind::BrownMushroomBlock => "brown_mushroom_block",
            ItemKind::RedMushroomBlock => "red_mushroom_block",
            ItemKind::MushroomStem => "mushroom_stem",
            ItemKind::IronBars => "iron_bars",
            ItemKind::Chain => "chain",
            ItemKind::GlassPane => "glass_pane",
            ItemKind::Melon => "melon",
            ItemKind::Vine => "vine",
            ItemKind::GlowLichen => "glow_lichen",
            ItemKind::BrickStairs => "brick_stairs",
            ItemKind::StoneBrickStairs => "stone_brick_stairs",
            ItemKind::Mycelium => "mycelium",
            ItemKind::LilyPad => "lily_pad",
            ItemKind::NetherBricks => "nether_bricks",
            ItemKind::CrackedNetherBricks => "cracked_nether_bricks",
            ItemKind::ChiseledNetherBricks => "chiseled_nether_bricks",
            ItemKind::NetherBrickFence => "nether_brick_fence",
            ItemKind::NetherBrickStairs => "nether_brick_stairs",
            ItemKind::EnchantingTable => "enchanting_table",
            ItemKind::EndPortalFrame => "end_portal_frame",
            ItemKind::EndStone => "end_stone",
            ItemKind::EndStoneBricks => "end_stone_bricks",
            ItemKind::DragonEgg => "dragon_egg",
            ItemKind::SandstoneStairs => "sandstone_stairs",
            ItemKind::EnderChest => "ender_chest",
            ItemKind::EmeraldBlock => "emerald_block",
            ItemKind::SpruceStairs => "spruce_stairs",
            ItemKind::BirchStairs => "birch_stairs",
            ItemKind::JungleStairs => "jungle_stairs",
            ItemKind::CrimsonStairs => "crimson_stairs",
            ItemKind::WarpedStairs => "warped_stairs",
            ItemKind::CommandBlock => "command_block",
            ItemKind::Beacon => "beacon",
            ItemKind::CobblestoneWall => "cobblestone_wall",
            ItemKind::MossyCobblestoneWall => "mossy_cobblestone_wall",
            ItemKind::BrickWall => "brick_wall",
            ItemKind::PrismarineWall => "prismarine_wall",
            ItemKind::RedSandstoneWall => "red_sandstone_wall",
            ItemKind::MossyStoneBrickWall => "mossy_stone_brick_wall",
            ItemKind::GraniteWall => "granite_wall",
            ItemKind::StoneBrickWall => "stone_brick_wall",
            ItemKind::NetherBrickWall => "nether_brick_wall",
            ItemKind::AndesiteWall => "andesite_wall",
            ItemKind::RedNetherBrickWall => "red_nether_brick_wall",
            ItemKind::SandstoneWall => "sandstone_wall",
            ItemKind::EndStoneBrickWall => "end_stone_brick_wall",
            ItemKind::DioriteWall => "diorite_wall",
            ItemKind::BlackstoneWall => "blackstone_wall",
            ItemKind::PolishedBlackstoneWall => "polished_blackstone_wall",
            ItemKind::PolishedBlackstoneBrickWall => "polished_blackstone_brick_wall",
            ItemKind::CobbledDeepslateWall => "cobbled_deepslate_wall",
            ItemKind::PolishedDeepslateWall => "polished_deepslate_wall",
            ItemKind::DeepslateBrickWall => "deepslate_brick_wall",
            ItemKind::DeepslateTileWall => "deepslate_tile_wall",
            ItemKind::Anvil => "anvil",
            ItemKind::ChippedAnvil => "chipped_anvil",
            ItemKind::DamagedAnvil => "damaged_anvil",
            ItemKind::ChiseledQuartzBlock => "chiseled_quartz_block",
            ItemKind::QuartzBlock => "quartz_block",
            ItemKind::QuartzBricks => "quartz_bricks",
            ItemKind::QuartzPillar => "quartz_pillar",
            ItemKind::QuartzStairs => "quartz_stairs",
            ItemKind::WhiteTerracotta => "white_terracotta",
            ItemKind::OrangeTerracotta => "orange_terracotta",
            ItemKind::MagentaTerracotta => "magenta_terracotta",
            ItemKind::LightBlueTerracotta => "light_blue_terracotta",
            ItemKind::YellowTerracotta => "yellow_terracotta",
            ItemKind::LimeTerracotta => "lime_terracotta",
            ItemKind::PinkTerracotta => "pink_terracotta",
            ItemKind::GrayTerracotta => "gray_terracotta",
            ItemKind::LightGrayTerracotta => "light_gray_terracotta",
            ItemKind::CyanTerracotta => "cyan_terracotta",
            ItemKind::PurpleTerracotta => "purple_terracotta",
            ItemKind::BlueTerracotta => "blue_terracotta",
            ItemKind::BrownTerracotta => "brown_terracotta",
            ItemKind::GreenTerracotta => "green_terracotta",
            ItemKind::RedTerracotta => "red_terracotta",
            ItemKind::BlackTerracotta => "black_terracotta",
            ItemKind::Barrier => "barrier",
            ItemKind::Light => "light",
            ItemKind::HayBlock => "hay_block",
            ItemKind::WhiteCarpet => "white_carpet",
            ItemKind::OrangeCarpet => "orange_carpet",
            ItemKind::MagentaCarpet => "magenta_carpet",
            ItemKind::LightBlueCarpet => "light_blue_carpet",
            ItemKind::YellowCarpet => "yellow_carpet",
            ItemKind::LimeCarpet => "lime_carpet",
            ItemKind::PinkCarpet => "pink_carpet",
            ItemKind::GrayCarpet => "gray_carpet",
            ItemKind::LightGrayCarpet => "light_gray_carpet",
            ItemKind::CyanCarpet => "cyan_carpet",
            ItemKind::PurpleCarpet => "purple_carpet",
            ItemKind::BlueCarpet => "blue_carpet",
            ItemKind::BrownCarpet => "brown_carpet",
            ItemKind::GreenCarpet => "green_carpet",
            ItemKind::RedCarpet => "red_carpet",
            ItemKind::BlackCarpet => "black_carpet",
            ItemKind::Terracotta => "terracotta",
            ItemKind::PackedIce => "packed_ice",
            ItemKind::AcaciaStairs => "acacia_stairs",
            ItemKind::DarkOakStairs => "dark_oak_stairs",
            ItemKind::DirtPath => "dirt_path",
            ItemKind::Sunflower => "sunflower",
            ItemKind::Lilac => "lilac",
            ItemKind::RoseBush => "rose_bush",
            ItemKind::Peony => "peony",
            ItemKind::TallGrass => "tall_grass",
            ItemKind::LargeFern => "large_fern",
            ItemKind::WhiteStainedGlass => "white_stained_glass",
            ItemKind::OrangeStainedGlass => "orange_stained_glass",
            ItemKind::MagentaStainedGlass => "magenta_stained_glass",
            ItemKind::LightBlueStainedGlass => "light_blue_stained_glass",
            ItemKind::YellowStainedGlass => "yellow_stained_glass",
            ItemKind::LimeStainedGlass => "lime_stained_glass",
            ItemKind::PinkStainedGlass => "pink_stained_glass",
            ItemKind::GrayStainedGlass => "gray_stained_glass",
            ItemKind::LightGrayStainedGlass => "light_gray_stained_glass",
            ItemKind::CyanStainedGlass => "cyan_stained_glass",
            ItemKind::PurpleStainedGlass => "purple_stained_glass",
            ItemKind::BlueStainedGlass => "blue_stained_glass",
            ItemKind::BrownStainedGlass => "brown_stained_glass",
            ItemKind::GreenStainedGlass => "green_stained_glass",
            ItemKind::RedStainedGlass => "red_stained_glass",
            ItemKind::BlackStainedGlass => "black_stained_glass",
            ItemKind::WhiteStainedGlassPane => "white_stained_glass_pane",
            ItemKind::OrangeStainedGlassPane => "orange_stained_glass_pane",
            ItemKind::MagentaStainedGlassPane => "magenta_stained_glass_pane",
            ItemKind::LightBlueStainedGlassPane => "light_blue_stained_glass_pane",
            ItemKind::YellowStainedGlassPane => "yellow_stained_glass_pane",
            ItemKind::LimeStainedGlassPane => "lime_stained_glass_pane",
            ItemKind::PinkStainedGlassPane => "pink_stained_glass_pane",
            ItemKind::GrayStainedGlassPane => "gray_stained_glass_pane",
            ItemKind::LightGrayStainedGlassPane => "light_gray_stained_glass_pane",
            ItemKind::CyanStainedGlassPane => "cyan_stained_glass_pane",
            ItemKind::PurpleStainedGlassPane => "purple_stained_glass_pane",
            ItemKind::BlueStainedGlassPane => "blue_stained_glass_pane",
            ItemKind::BrownStainedGlassPane => "brown_stained_glass_pane",
            ItemKind::GreenStainedGlassPane => "green_stained_glass_pane",
            ItemKind::RedStainedGlassPane => "red_stained_glass_pane",
            ItemKind::BlackStainedGlassPane => "black_stained_glass_pane",
            ItemKind::Prismarine => "prismarine",
            ItemKind::PrismarineBricks => "prismarine_bricks",
            ItemKind::DarkPrismarine => "dark_prismarine",
            ItemKind::PrismarineStairs => "prismarine_stairs",
            ItemKind::PrismarineBrickStairs => "prismarine_brick_stairs",
            ItemKind::DarkPrismarineStairs => "dark_prismarine_stairs",
            ItemKind::SeaLantern => "sea_lantern",
            ItemKind::RedSandstone => "red_sandstone",
            ItemKind::ChiseledRedSandstone => "chiseled_red_sandstone",
            ItemKind::CutRedSandstone => "cut_red_sandstone",
            ItemKind::RedSandstoneStairs => "red_sandstone_stairs",
            ItemKind::RepeatingCommandBlock => "repeating_command_block",
            ItemKind::ChainCommandBlock => "chain_command_block",
            ItemKind::MagmaBlock => "magma_block",
            ItemKind::NetherWartBlock => "nether_wart_block",
            ItemKind::WarpedWartBlock => "warped_wart_block",
            ItemKind::RedNetherBricks => "red_nether_bricks",
            ItemKind::BoneBlock => "bone_block",
            ItemKind::StructureVoid => "structure_void",
            ItemKind::ShulkerBox => "shulker_box",
            ItemKind::WhiteShulkerBox => "white_shulker_box",
            ItemKind::OrangeShulkerBox => "orange_shulker_box",
            ItemKind::MagentaShulkerBox => "magenta_shulker_box",
            ItemKind::LightBlueShulkerBox => "light_blue_shulker_box",
            ItemKind::YellowShulkerBox => "yellow_shulker_box",
            ItemKind::LimeShulkerBox => "lime_shulker_box",
            ItemKind::PinkShulkerBox => "pink_shulker_box",
            ItemKind::GrayShulkerBox => "gray_shulker_box",
            ItemKind::LightGrayShulkerBox => "light_gray_shulker_box",
            ItemKind::CyanShulkerBox => "cyan_shulker_box",
            ItemKind::PurpleShulkerBox => "purple_shulker_box",
            ItemKind::BlueShulkerBox => "blue_shulker_box",
            ItemKind::BrownShulkerBox => "brown_shulker_box",
            ItemKind::GreenShulkerBox => "green_shulker_box",
            ItemKind::RedShulkerBox => "red_shulker_box",
            ItemKind::BlackShulkerBox => "black_shulker_box",
            ItemKind::WhiteGlazedTerracotta => "white_glazed_terracotta",
            ItemKind::OrangeGlazedTerracotta => "orange_glazed_terracotta",
            ItemKind::MagentaGlazedTerracotta => "magenta_glazed_terracotta",
            ItemKind::LightBlueGlazedTerracotta => "light_blue_glazed_terracotta",
            ItemKind::YellowGlazedTerracotta => "yellow_glazed_terracotta",
            ItemKind::LimeGlazedTerracotta => "lime_glazed_terracotta",
            ItemKind::PinkGlazedTerracotta => "pink_glazed_terracotta",
            ItemKind::GrayGlazedTerracotta => "gray_glazed_terracotta",
            ItemKind::LightGrayGlazedTerracotta => "light_gray_glazed_terracotta",
            ItemKind::CyanGlazedTerracotta => "cyan_glazed_terracotta",
            ItemKind::PurpleGlazedTerracotta => "purple_glazed_terracotta",
            ItemKind::BlueGlazedTerracotta => "blue_glazed_terracotta",
            ItemKind::BrownGlazedTerracotta => "brown_glazed_terracotta",
            ItemKind::GreenGlazedTerracotta => "green_glazed_terracotta",
            ItemKind::RedGlazedTerracotta => "red_glazed_terracotta",
            ItemKind::BlackGlazedTerracotta => "black_glazed_terracotta",
            ItemKind::WhiteConcrete => "white_concrete",
            ItemKind::OrangeConcrete => "orange_concrete",
            ItemKind::MagentaConcrete => "magenta_concrete",
            ItemKind::LightBlueConcrete => "light_blue_concrete",
            ItemKind::YellowConcrete => "yellow_concrete",
            ItemKind::LimeConcrete => "lime_concrete",
            ItemKind::PinkConcrete => "pink_concrete",
            ItemKind::GrayConcrete => "gray_concrete",
            ItemKind::LightGrayConcrete => "light_gray_concrete",
            ItemKind::CyanConcrete => "cyan_concrete",
            ItemKind::PurpleConcrete => "purple_concrete",
            ItemKind::BlueConcrete => "blue_concrete",
            ItemKind::BrownConcrete => "brown_concrete",
            ItemKind::GreenConcrete => "green_concrete",
            ItemKind::RedConcrete => "red_concrete",
            ItemKind::BlackConcrete => "black_concrete",
            ItemKind::WhiteConcretePowder => "white_concrete_powder",
            ItemKind::OrangeConcretePowder => "orange_concrete_powder",
            ItemKind::MagentaConcretePowder => "magenta_concrete_powder",
            ItemKind::LightBlueConcretePowder => "light_blue_concrete_powder",
            ItemKind::YellowConcretePowder => "yellow_concrete_powder",
            ItemKind::LimeConcretePowder => "lime_concrete_powder",
            ItemKind::PinkConcretePowder => "pink_concrete_powder",
            ItemKind::GrayConcretePowder => "gray_concrete_powder",
            ItemKind::LightGrayConcretePowder => "light_gray_concrete_powder",
            ItemKind::CyanConcretePowder => "cyan_concrete_powder",
            ItemKind::PurpleConcretePowder => "purple_concrete_powder",
            ItemKind::BlueConcretePowder => "blue_concrete_powder",
            ItemKind::BrownConcretePowder => "brown_concrete_powder",
            ItemKind::GreenConcretePowder => "green_concrete_powder",
            ItemKind::RedConcretePowder => "red_concrete_powder",
            ItemKind::BlackConcretePowder => "black_concrete_powder",
            ItemKind::TurtleEgg => "turtle_egg",
            ItemKind::DeadTubeCoralBlock => "dead_tube_coral_block",
            ItemKind::DeadBrainCoralBlock => "dead_brain_coral_block",
            ItemKind::DeadBubbleCoralBlock => "dead_bubble_coral_block",
            ItemKind::DeadFireCoralBlock => "dead_fire_coral_block",
            ItemKind::DeadHornCoralBlock => "dead_horn_coral_block",
            ItemKind::TubeCoralBlock => "tube_coral_block",
            ItemKind::BrainCoralBlock => "brain_coral_block",
            ItemKind::BubbleCoralBlock => "bubble_coral_block",
            ItemKind::FireCoralBlock => "fire_coral_block",
            ItemKind::HornCoralBlock => "horn_coral_block",
            ItemKind::TubeCoral => "tube_coral",
            ItemKind::BrainCoral => "brain_coral",
            ItemKind::BubbleCoral => "bubble_coral",
            ItemKind::FireCoral => "fire_coral",
            ItemKind::HornCoral => "horn_coral",
            ItemKind::DeadBrainCoral => "dead_brain_coral",
            ItemKind::DeadBubbleCoral => "dead_bubble_coral",
            ItemKind::DeadFireCoral => "dead_fire_coral",
            ItemKind::DeadHornCoral => "dead_horn_coral",
            ItemKind::DeadTubeCoral => "dead_tube_coral",
            ItemKind::TubeCoralFan => "tube_coral_fan",
            ItemKind::BrainCoralFan => "brain_coral_fan",
            ItemKind::BubbleCoralFan => "bubble_coral_fan",
            ItemKind::FireCoralFan => "fire_coral_fan",
            ItemKind::HornCoralFan => "horn_coral_fan",
            ItemKind::DeadTubeCoralFan => "dead_tube_coral_fan",
            ItemKind::DeadBrainCoralFan => "dead_brain_coral_fan",
            ItemKind::DeadBubbleCoralFan => "dead_bubble_coral_fan",
            ItemKind::DeadFireCoralFan => "dead_fire_coral_fan",
            ItemKind::DeadHornCoralFan => "dead_horn_coral_fan",
            ItemKind::BlueIce => "blue_ice",
            ItemKind::Conduit => "conduit",
            ItemKind::PolishedGraniteStairs => "polished_granite_stairs",
            ItemKind::SmoothRedSandstoneStairs => "smooth_red_sandstone_stairs",
            ItemKind::MossyStoneBrickStairs => "mossy_stone_brick_stairs",
            ItemKind::PolishedDioriteStairs => "polished_diorite_stairs",
            ItemKind::MossyCobblestoneStairs => "mossy_cobblestone_stairs",
            ItemKind::EndStoneBrickStairs => "end_stone_brick_stairs",
            ItemKind::StoneStairs => "stone_stairs",
            ItemKind::SmoothSandstoneStairs => "smooth_sandstone_stairs",
            ItemKind::SmoothQuartzStairs => "smooth_quartz_stairs",
            ItemKind::GraniteStairs => "granite_stairs",
            ItemKind::AndesiteStairs => "andesite_stairs",
            ItemKind::RedNetherBrickStairs => "red_nether_brick_stairs",
            ItemKind::PolishedAndesiteStairs => "polished_andesite_stairs",
            ItemKind::DioriteStairs => "diorite_stairs",
            ItemKind::CobbledDeepslateStairs => "cobbled_deepslate_stairs",
            ItemKind::PolishedDeepslateStairs => "polished_deepslate_stairs",
            ItemKind::DeepslateBrickStairs => "deepslate_brick_stairs",
            ItemKind::DeepslateTileStairs => "deepslate_tile_stairs",
            ItemKind::PolishedGraniteSlab => "polished_granite_slab",
            ItemKind::SmoothRedSandstoneSlab => "smooth_red_sandstone_slab",
            ItemKind::MossyStoneBrickSlab => "mossy_stone_brick_slab",
            ItemKind::PolishedDioriteSlab => "polished_diorite_slab",
            ItemKind::MossyCobblestoneSlab => "mossy_cobblestone_slab",
            ItemKind::EndStoneBrickSlab => "end_stone_brick_slab",
            ItemKind::SmoothSandstoneSlab => "smooth_sandstone_slab",
            ItemKind::SmoothQuartzSlab => "smooth_quartz_slab",
            ItemKind::GraniteSlab => "granite_slab",
            ItemKind::AndesiteSlab => "andesite_slab",
            ItemKind::RedNetherBrickSlab => "red_nether_brick_slab",
            ItemKind::PolishedAndesiteSlab => "polished_andesite_slab",
            ItemKind::DioriteSlab => "diorite_slab",
            ItemKind::CobbledDeepslateSlab => "cobbled_deepslate_slab",
            ItemKind::PolishedDeepslateSlab => "polished_deepslate_slab",
            ItemKind::DeepslateBrickSlab => "deepslate_brick_slab",
            ItemKind::DeepslateTileSlab => "deepslate_tile_slab",
            ItemKind::Scaffolding => "scaffolding",
            ItemKind::Redstone => "redstone",
            ItemKind::RedstoneTorch => "redstone_torch",
            ItemKind::RedstoneBlock => "redstone_block",
            ItemKind::Repeater => "repeater",
            ItemKind::Comparator => "comparator",
            ItemKind::Piston => "piston",
            ItemKind::StickyPiston => "sticky_piston",
            ItemKind::SlimeBlock => "slime_block",
            ItemKind::HoneyBlock => "honey_block",
            ItemKind::Observer => "observer",
            ItemKind::Hopper => "hopper",
            ItemKind::Dispenser => "dispenser",
            ItemKind::Dropper => "dropper",
            ItemKind::Lectern => "lectern",
            ItemKind::Target => "target",
            ItemKind::Lever => "lever",
            ItemKind::LightningRod => "lightning_rod",
            ItemKind::DaylightDetector => "daylight_detector",
            ItemKind::SculkSensor => "sculk_sensor",
            ItemKind::TripwireHook => "tripwire_hook",
            ItemKind::TrappedChest => "trapped_chest",
            ItemKind::Tnt => "tnt",
            ItemKind::RedstoneLamp => "redstone_lamp",
            ItemKind::NoteBlock => "note_block",
            ItemKind::StoneButton => "stone_button",
            ItemKind::PolishedBlackstoneButton => "polished_blackstone_button",
            ItemKind::OakButton => "oak_button",
            ItemKind::SpruceButton => "spruce_button",
            ItemKind::BirchButton => "birch_button",
            ItemKind::JungleButton => "jungle_button",
            ItemKind::AcaciaButton => "acacia_button",
            ItemKind::DarkOakButton => "dark_oak_button",
            ItemKind::CrimsonButton => "crimson_button",
            ItemKind::WarpedButton => "warped_button",
            ItemKind::StonePressurePlate => "stone_pressure_plate",
            ItemKind::PolishedBlackstonePressurePlate => "polished_blackstone_pressure_plate",
            ItemKind::LightWeightedPressurePlate => "light_weighted_pressure_plate",
            ItemKind::HeavyWeightedPressurePlate => "heavy_weighted_pressure_plate",
            ItemKind::OakPressurePlate => "oak_pressure_plate",
            ItemKind::SprucePressurePlate => "spruce_pressure_plate",
            ItemKind::BirchPressurePlate => "birch_pressure_plate",
            ItemKind::JunglePressurePlate => "jungle_pressure_plate",
            ItemKind::AcaciaPressurePlate => "acacia_pressure_plate",
            ItemKind::DarkOakPressurePlate => "dark_oak_pressure_plate",
            ItemKind::CrimsonPressurePlate => "crimson_pressure_plate",
            ItemKind::WarpedPressurePlate => "warped_pressure_plate",
            ItemKind::IronDoor => "iron_door",
            ItemKind::OakDoor => "oak_door",
            ItemKind::SpruceDoor => "spruce_door",
            ItemKind::BirchDoor => "birch_door",
            ItemKind::JungleDoor => "jungle_door",
            ItemKind::AcaciaDoor => "acacia_door",
            ItemKind::DarkOakDoor => "dark_oak_door",
            ItemKind::CrimsonDoor => "crimson_door",
            ItemKind::WarpedDoor => "warped_door",
            ItemKind::IronTrapdoor => "iron_trapdoor",
            ItemKind::OakTrapdoor => "oak_trapdoor",
            ItemKind::SpruceTrapdoor => "spruce_trapdoor",
            ItemKind::BirchTrapdoor => "birch_trapdoor",
            ItemKind::JungleTrapdoor => "jungle_trapdoor",
            ItemKind::AcaciaTrapdoor => "acacia_trapdoor",
            ItemKind::DarkOakTrapdoor => "dark_oak_trapdoor",
            ItemKind::CrimsonTrapdoor => "crimson_trapdoor",
            ItemKind::WarpedTrapdoor => "warped_trapdoor",
            ItemKind::OakFenceGate => "oak_fence_gate",
            ItemKind::SpruceFenceGate => "spruce_fence_gate",
            ItemKind::BirchFenceGate => "birch_fence_gate",
            ItemKind::JungleFenceGate => "jungle_fence_gate",
            ItemKind::AcaciaFenceGate => "acacia_fence_gate",
            ItemKind::DarkOakFenceGate => "dark_oak_fence_gate",
            ItemKind::CrimsonFenceGate => "crimson_fence_gate",
            ItemKind::WarpedFenceGate => "warped_fence_gate",
            ItemKind::PoweredRail => "powered_rail",
            ItemKind::DetectorRail => "detector_rail",
            ItemKind::Rail => "rail",
            ItemKind::ActivatorRail => "activator_rail",
            ItemKind::Saddle => "saddle",
            ItemKind::Minecart => "minecart",
            ItemKind::ChestMinecart => "chest_minecart",
            ItemKind::FurnaceMinecart => "furnace_minecart",
            ItemKind::TntMinecart => "tnt_minecart",
            ItemKind::HopperMinecart => "hopper_minecart",
            ItemKind::CarrotOnAStick => "carrot_on_a_stick",
            ItemKind::WarpedFungusOnAStick => "warped_fungus_on_a_stick",
            ItemKind::Elytra => "elytra",
            ItemKind::OakBoat => "oak_boat",
            ItemKind::SpruceBoat => "spruce_boat",
            ItemKind::BirchBoat => "birch_boat",
            ItemKind::JungleBoat => "jungle_boat",
            ItemKind::AcaciaBoat => "acacia_boat",
            ItemKind::DarkOakBoat => "dark_oak_boat",
            ItemKind::StructureBlock => "structure_block",
            ItemKind::Jigsaw => "jigsaw",
            ItemKind::TurtleHelmet => "turtle_helmet",
            ItemKind::Scute => "scute",
            ItemKind::FlintAndSteel => "flint_and_steel",
            ItemKind::Apple => "apple",
            ItemKind::Bow => "bow",
            ItemKind::Arrow => "arrow",
            ItemKind::Coal => "coal",
            ItemKind::Charcoal => "charcoal",
            ItemKind::Diamond => "diamond",
            ItemKind::Emerald => "emerald",
            ItemKind::LapisLazuli => "lapis_lazuli",
            ItemKind::Quartz => "quartz",
            ItemKind::AmethystShard => "amethyst_shard",
            ItemKind::RawIron => "raw_iron",
            ItemKind::IronIngot => "iron_ingot",
            ItemKind::RawCopper => "raw_copper",
            ItemKind::CopperIngot => "copper_ingot",
            ItemKind::RawGold => "raw_gold",
            ItemKind::GoldIngot => "gold_ingot",
            ItemKind::NetheriteIngot => "netherite_ingot",
            ItemKind::NetheriteScrap => "netherite_scrap",
            ItemKind::WoodenSword => "wooden_sword",
            ItemKind::WoodenShovel => "wooden_shovel",
            ItemKind::WoodenPickaxe => "wooden_pickaxe",
            ItemKind::WoodenAxe => "wooden_axe",
            ItemKind::WoodenHoe => "wooden_hoe",
            ItemKind::StoneSword => "stone_sword",
            ItemKind::StoneShovel => "stone_shovel",
            ItemKind::StonePickaxe => "stone_pickaxe",
            ItemKind::StoneAxe => "stone_axe",
            ItemKind::StoneHoe => "stone_hoe",
            ItemKind::GoldenSword => "golden_sword",
            ItemKind::GoldenShovel => "golden_shovel",
            ItemKind::GoldenPickaxe => "golden_pickaxe",
            ItemKind::GoldenAxe => "golden_axe",
            ItemKind::GoldenHoe => "golden_hoe",
            ItemKind::IronSword => "iron_sword",
            ItemKind::IronShovel => "iron_shovel",
            ItemKind::IronPickaxe => "iron_pickaxe",
            ItemKind::IronAxe => "iron_axe",
            ItemKind::IronHoe => "iron_hoe",
            ItemKind::DiamondSword => "diamond_sword",
            ItemKind::DiamondShovel => "diamond_shovel",
            ItemKind::DiamondPickaxe => "diamond_pickaxe",
            ItemKind::DiamondAxe => "diamond_axe",
            ItemKind::DiamondHoe => "diamond_hoe",
            ItemKind::NetheriteSword => "netherite_sword",
            ItemKind::NetheriteShovel => "netherite_shovel",
            ItemKind::NetheritePickaxe => "netherite_pickaxe",
            ItemKind::NetheriteAxe => "netherite_axe",
            ItemKind::NetheriteHoe => "netherite_hoe",
            ItemKind::Stick => "stick",
            ItemKind::Bowl => "bowl",
            ItemKind::MushroomStew => "mushroom_stew",
            ItemKind::String => "string",
            ItemKind::Feather => "feather",
            ItemKind::Gunpowder => "gunpowder",
            ItemKind::WheatSeeds => "wheat_seeds",
            ItemKind::Wheat => "wheat",
            ItemKind::Bread => "bread",
            ItemKind::LeatherHelmet => "leather_helmet",
            ItemKind::LeatherChestplate => "leather_chestplate",
            ItemKind::LeatherLeggings => "leather_leggings",
            ItemKind::LeatherBoots => "leather_boots",
            ItemKind::ChainmailHelmet => "chainmail_helmet",
            ItemKind::ChainmailChestplate => "chainmail_chestplate",
            ItemKind::ChainmailLeggings => "chainmail_leggings",
            ItemKind::ChainmailBoots => "chainmail_boots",
            ItemKind::IronHelmet => "iron_helmet",
            ItemKind::IronChestplate => "iron_chestplate",
            ItemKind::IronLeggings => "iron_leggings",
            ItemKind::IronBoots => "iron_boots",
            ItemKind::DiamondHelmet => "diamond_helmet",
            ItemKind::DiamondChestplate => "diamond_chestplate",
            ItemKind::DiamondLeggings => "diamond_leggings",
            ItemKind::DiamondBoots => "diamond_boots",
            ItemKind::GoldenHelmet => "golden_helmet",
            ItemKind::GoldenChestplate => "golden_chestplate",
            ItemKind::GoldenLeggings => "golden_leggings",
            ItemKind::GoldenBoots => "golden_boots",
            ItemKind::NetheriteHelmet => "netherite_helmet",
            ItemKind::NetheriteChestplate => "netherite_chestplate",
            ItemKind::NetheriteLeggings => "netherite_leggings",
            ItemKind::NetheriteBoots => "netherite_boots",
            ItemKind::Flint => "flint",
            ItemKind::Porkchop => "porkchop",
            ItemKind::CookedPorkchop => "cooked_porkchop",
            ItemKind::Painting => "painting",
            ItemKind::GoldenApple => "golden_apple",
            ItemKind::EnchantedGoldenApple => "enchanted_golden_apple",
            ItemKind::OakSign => "oak_sign",
            ItemKind::SpruceSign => "spruce_sign",
            ItemKind::BirchSign => "birch_sign",
            ItemKind::JungleSign => "jungle_sign",
            ItemKind::AcaciaSign => "acacia_sign",
            ItemKind::DarkOakSign => "dark_oak_sign",
            ItemKind::CrimsonSign => "crimson_sign",
            ItemKind::WarpedSign => "warped_sign",
            ItemKind::Bucket => "bucket",
            ItemKind::WaterBucket => "water_bucket",
            ItemKind::LavaBucket => "lava_bucket",
            ItemKind::PowderSnowBucket => "powder_snow_bucket",
            ItemKind::Snowball => "snowball",
            ItemKind::Leather => "leather",
            ItemKind::MilkBucket => "milk_bucket",
            ItemKind::PufferfishBucket => "pufferfish_bucket",
            ItemKind::SalmonBucket => "salmon_bucket",
            ItemKind::CodBucket => "cod_bucket",
            ItemKind::TropicalFishBucket => "tropical_fish_bucket",
            ItemKind::AxolotlBucket => "axolotl_bucket",
            ItemKind::Brick => "brick",
            ItemKind::ClayBall => "clay_ball",
            ItemKind::DriedKelpBlock => "dried_kelp_block",
            ItemKind::Paper => "paper",
            ItemKind::Book => "book",
            ItemKind::SlimeBall => "slime_ball",
            ItemKind::Egg => "egg",
            ItemKind::Compass => "compass",
            ItemKind::Bundle => "bundle",
            ItemKind::FishingRod => "fishing_rod",
            ItemKind::Clock => "clock",
            ItemKind::Spyglass => "spyglass",
            ItemKind::GlowstoneDust => "glowstone_dust",
            ItemKind::Cod => "cod",
            ItemKind::Salmon => "salmon",
            ItemKind::TropicalFish => "tropical_fish",
            ItemKind::Pufferfish => "pufferfish",
            ItemKind::CookedCod => "cooked_cod",
            ItemKind::CookedSalmon => "cooked_salmon",
            ItemKind::InkSac => "ink_sac",
            ItemKind::GlowInkSac => "glow_ink_sac",
            ItemKind::CocoaBeans => "cocoa_beans",
            ItemKind::WhiteDye => "white_dye",
            ItemKind::OrangeDye => "orange_dye",
            ItemKind::MagentaDye => "magenta_dye",
            ItemKind::LightBlueDye => "light_blue_dye",
            ItemKind::YellowDye => "yellow_dye",
            ItemKind::LimeDye => "lime_dye",
            ItemKind::PinkDye => "pink_dye",
            ItemKind::GrayDye => "gray_dye",
            ItemKind::LightGrayDye => "light_gray_dye",
            ItemKind::CyanDye => "cyan_dye",
            ItemKind::PurpleDye => "purple_dye",
            ItemKind::BlueDye => "blue_dye",
            ItemKind::BrownDye => "brown_dye",
            ItemKind::GreenDye => "green_dye",
            ItemKind::RedDye => "red_dye",
            ItemKind::BlackDye => "black_dye",
            ItemKind::BoneMeal => "bone_meal",
            ItemKind::Bone => "bone",
            ItemKind::Sugar => "sugar",
            ItemKind::Cake => "cake",
            ItemKind::WhiteBed => "white_bed",
            ItemKind::OrangeBed => "orange_bed",
            ItemKind::MagentaBed => "magenta_bed",
            ItemKind::LightBlueBed => "light_blue_bed",
            ItemKind::YellowBed => "yellow_bed",
            ItemKind::LimeBed => "lime_bed",
            ItemKind::PinkBed => "pink_bed",
            ItemKind::GrayBed => "gray_bed",
            ItemKind::LightGrayBed => "light_gray_bed",
            ItemKind::CyanBed => "cyan_bed",
            ItemKind::PurpleBed => "purple_bed",
            ItemKind::BlueBed => "blue_bed",
            ItemKind::BrownBed => "brown_bed",
            ItemKind::GreenBed => "green_bed",
            ItemKind::RedBed => "red_bed",
            ItemKind::BlackBed => "black_bed",
            ItemKind::Cookie => "cookie",
            ItemKind::FilledMap => "filled_map",
            ItemKind::Shears => "shears",
            ItemKind::MelonSlice => "melon_slice",
            ItemKind::DriedKelp => "dried_kelp",
            ItemKind::PumpkinSeeds => "pumpkin_seeds",
            ItemKind::MelonSeeds => "melon_seeds",
            ItemKind::Beef => "beef",
            ItemKind::CookedBeef => "cooked_beef",
            ItemKind::Chicken => "chicken",
            ItemKind::CookedChicken => "cooked_chicken",
            ItemKind::RottenFlesh => "rotten_flesh",
            ItemKind::EnderPearl => "ender_pearl",
            ItemKind::BlazeRod => "blaze_rod",
            ItemKind::GhastTear => "ghast_tear",
            ItemKind::GoldNugget => "gold_nugget",
            ItemKind::NetherWart => "nether_wart",
            ItemKind::Potion => "potion",
            ItemKind::GlassBottle => "glass_bottle",
            ItemKind::SpiderEye => "spider_eye",
            ItemKind::FermentedSpiderEye => "fermented_spider_eye",
            ItemKind::BlazePowder => "blaze_powder",
            ItemKind::MagmaCream => "magma_cream",
            ItemKind::BrewingStand => "brewing_stand",
            ItemKind::Cauldron => "cauldron",
            ItemKind::EnderEye => "ender_eye",
            ItemKind::GlisteringMelonSlice => "glistering_melon_slice",
            ItemKind::AxolotlSpawnEgg => "axolotl_spawn_egg",
            ItemKind::BatSpawnEgg => "bat_spawn_egg",
            ItemKind::BeeSpawnEgg => "bee_spawn_egg",
            ItemKind::BlazeSpawnEgg => "blaze_spawn_egg",
            ItemKind::CatSpawnEgg => "cat_spawn_egg",
            ItemKind::CaveSpiderSpawnEgg => "cave_spider_spawn_egg",
            ItemKind::ChickenSpawnEgg => "chicken_spawn_egg",
            ItemKind::CodSpawnEgg => "cod_spawn_egg",
            ItemKind::CowSpawnEgg => "cow_spawn_egg",
            ItemKind::CreeperSpawnEgg => "creeper_spawn_egg",
            ItemKind::DolphinSpawnEgg => "dolphin_spawn_egg",
            ItemKind::DonkeySpawnEgg => "donkey_spawn_egg",
            ItemKind::DrownedSpawnEgg => "drowned_spawn_egg",
            ItemKind::ElderGuardianSpawnEgg => "elder_guardian_spawn_egg",
            ItemKind::EndermanSpawnEgg => "enderman_spawn_egg",
            ItemKind::EndermiteSpawnEgg => "endermite_spawn_egg",
            ItemKind::EvokerSpawnEgg => "evoker_spawn_egg",
            ItemKind::FoxSpawnEgg => "fox_spawn_egg",
            ItemKind::GhastSpawnEgg => "ghast_spawn_egg",
            ItemKind::GlowSquidSpawnEgg => "glow_squid_spawn_egg",
            ItemKind::GoatSpawnEgg => "goat_spawn_egg",
            ItemKind::GuardianSpawnEgg => "guardian_spawn_egg",
            ItemKind::HoglinSpawnEgg => "hoglin_spawn_egg",
            ItemKind::HorseSpawnEgg => "horse_spawn_egg",
            ItemKind::HuskSpawnEgg => "husk_spawn_egg",
            ItemKind::LlamaSpawnEgg => "llama_spawn_egg",
            ItemKind::MagmaCubeSpawnEgg => "magma_cube_spawn_egg",
            ItemKind::MooshroomSpawnEgg => "mooshroom_spawn_egg",
            ItemKind::MuleSpawnEgg => "mule_spawn_egg",
            ItemKind::OcelotSpawnEgg => "ocelot_spawn_egg",
            ItemKind::PandaSpawnEgg => "panda_spawn_egg",
            ItemKind::ParrotSpawnEgg => "parrot_spawn_egg",
            ItemKind::PhantomSpawnEgg => "phantom_spawn_egg",
            ItemKind::PigSpawnEgg => "pig_spawn_egg",
            ItemKind::PiglinSpawnEgg => "piglin_spawn_egg",
            ItemKind::PiglinBruteSpawnEgg => "piglin_brute_spawn_egg",
            ItemKind::PillagerSpawnEgg => "pillager_spawn_egg",
            ItemKind::PolarBearSpawnEgg => "polar_bear_spawn_egg",
            ItemKind::PufferfishSpawnEgg => "pufferfish_spawn_egg",
            ItemKind::RabbitSpawnEgg => "rabbit_spawn_egg",
            ItemKind::RavagerSpawnEgg => "ravager_spawn_egg",
            ItemKind::SalmonSpawnEgg => "salmon_spawn_egg",
            ItemKind::SheepSpawnEgg => "sheep_spawn_egg",
            ItemKind::ShulkerSpawnEgg => "shulker_spawn_egg",
            ItemKind::SilverfishSpawnEgg => "silverfish_spawn_egg",
            ItemKind::SkeletonSpawnEgg => "skeleton_spawn_egg",
            ItemKind::SkeletonHorseSpawnEgg => "skeleton_horse_spawn_egg",
            ItemKind::SlimeSpawnEgg => "slime_spawn_egg",
            ItemKind::SpiderSpawnEgg => "spider_spawn_egg",
            ItemKind::SquidSpawnEgg => "squid_spawn_egg",
            ItemKind::StraySpawnEgg => "stray_spawn_egg",
            ItemKind::StriderSpawnEgg => "strider_spawn_egg",
            ItemKind::TraderLlamaSpawnEgg => "trader_llama_spawn_egg",
            ItemKind::TropicalFishSpawnEgg => "tropical_fish_spawn_egg",
            ItemKind::TurtleSpawnEgg => "turtle_spawn_egg",
            ItemKind::VexSpawnEgg => "vex_spawn_egg",
            ItemKind::VillagerSpawnEgg => "villager_spawn_egg",
            ItemKind::VindicatorSpawnEgg => "vindicator_spawn_egg",
            ItemKind::WanderingTraderSpawnEgg => "wandering_trader_spawn_egg",
            ItemKind::WitchSpawnEgg => "witch_spawn_egg",
            ItemKind::WitherSkeletonSpawnEgg => "wither_skeleton_spawn_egg",
            ItemKind::WolfSpawnEgg => "wolf_spawn_egg",
            ItemKind::ZoglinSpawnEgg => "zoglin_spawn_egg",
            ItemKind::ZombieSpawnEgg => "zombie_spawn_egg",
            ItemKind::ZombieHorseSpawnEgg => "zombie_horse_spawn_egg",
            ItemKind::ZombieVillagerSpawnEgg => "zombie_villager_spawn_egg",
            ItemKind::ZombifiedPiglinSpawnEgg => "zombified_piglin_spawn_egg",
            ItemKind::ExperienceBottle => "experience_bottle",
            ItemKind::FireCharge => "fire_charge",
            ItemKind::WritableBook => "writable_book",
            ItemKind::WrittenBook => "written_book",
            ItemKind::ItemFrame => "item_frame",
            ItemKind::GlowItemFrame => "glow_item_frame",
            ItemKind::FlowerPot => "flower_pot",
            ItemKind::Carrot => "carrot",
            ItemKind::Potato => "potato",
            ItemKind::BakedPotato => "baked_potato",
            ItemKind::PoisonousPotato => "poisonous_potato",
            ItemKind::Map => "map",
            ItemKind::GoldenCarrot => "golden_carrot",
            ItemKind::SkeletonSkull => "skeleton_skull",
            ItemKind::WitherSkeletonSkull => "wither_skeleton_skull",
            ItemKind::PlayerHead => "player_head",
            ItemKind::ZombieHead => "zombie_head",
            ItemKind::CreeperHead => "creeper_head",
            ItemKind::DragonHead => "dragon_head",
            ItemKind::NetherStar => "nether_star",
            ItemKind::PumpkinPie => "pumpkin_pie",
            ItemKind::FireworkRocket => "firework_rocket",
            ItemKind::FireworkStar => "firework_star",
            ItemKind::EnchantedBook => "enchanted_book",
            ItemKind::NetherBrick => "nether_brick",
            ItemKind::PrismarineShard => "prismarine_shard",
            ItemKind::PrismarineCrystals => "prismarine_crystals",
            ItemKind::Rabbit => "rabbit",
            ItemKind::CookedRabbit => "cooked_rabbit",
            ItemKind::RabbitStew => "rabbit_stew",
            ItemKind::RabbitFoot => "rabbit_foot",
            ItemKind::RabbitHide => "rabbit_hide",
            ItemKind::ArmorStand => "armor_stand",
            ItemKind::IronHorseArmor => "iron_horse_armor",
            ItemKind::GoldenHorseArmor => "golden_horse_armor",
            ItemKind::DiamondHorseArmor => "diamond_horse_armor",
            ItemKind::LeatherHorseArmor => "leather_horse_armor",
            ItemKind::Lead => "lead",
            ItemKind::NameTag => "name_tag",
            ItemKind::CommandBlockMinecart => "command_block_minecart",
            ItemKind::Mutton => "mutton",
            ItemKind::CookedMutton => "cooked_mutton",
            ItemKind::WhiteBanner => "white_banner",
            ItemKind::OrangeBanner => "orange_banner",
            ItemKind::MagentaBanner => "magenta_banner",
            ItemKind::LightBlueBanner => "light_blue_banner",
            ItemKind::YellowBanner => "yellow_banner",
            ItemKind::LimeBanner => "lime_banner",
            ItemKind::PinkBanner => "pink_banner",
            ItemKind::GrayBanner => "gray_banner",
            ItemKind::LightGrayBanner => "light_gray_banner",
            ItemKind::CyanBanner => "cyan_banner",
            ItemKind::PurpleBanner => "purple_banner",
            ItemKind::BlueBanner => "blue_banner",
            ItemKind::BrownBanner => "brown_banner",
            ItemKind::GreenBanner => "green_banner",
            ItemKind::RedBanner => "red_banner",
            ItemKind::BlackBanner => "black_banner",
            ItemKind::EndCrystal => "end_crystal",
            ItemKind::ChorusFruit => "chorus_fruit",
            ItemKind::PoppedChorusFruit => "popped_chorus_fruit",
            ItemKind::Beetroot => "beetroot",
            ItemKind::BeetrootSeeds => "beetroot_seeds",
            ItemKind::BeetrootSoup => "beetroot_soup",
            ItemKind::DragonBreath => "dragon_breath",
            ItemKind::SplashPotion => "splash_potion",
            ItemKind::SpectralArrow => "spectral_arrow",
            ItemKind::TippedArrow => "tipped_arrow",
            ItemKind::LingeringPotion => "lingering_potion",
            ItemKind::Shield => "shield",
            ItemKind::TotemOfUndying => "totem_of_undying",
            ItemKind::ShulkerShell => "shulker_shell",
            ItemKind::IronNugget => "iron_nugget",
            ItemKind::KnowledgeBook => "knowledge_book",
            ItemKind::DebugStick => "debug_stick",
            ItemKind::MusicDisc13 => "music_disc_13",
            ItemKind::MusicDiscCat => "music_disc_cat",
            ItemKind::MusicDiscBlocks => "music_disc_blocks",
            ItemKind::MusicDiscChirp => "music_disc_chirp",
            ItemKind::MusicDiscFar => "music_disc_far",
            ItemKind::MusicDiscMall => "music_disc_mall",
            ItemKind::MusicDiscMellohi => "music_disc_mellohi",
            ItemKind::MusicDiscStal => "music_disc_stal",
            ItemKind::MusicDiscStrad => "music_disc_strad",
            ItemKind::MusicDiscWard => "music_disc_ward",
            ItemKind::MusicDisc11 => "music_disc_11",
            ItemKind::MusicDiscWait => "music_disc_wait",
            ItemKind::MusicDiscOtherside => "music_disc_otherside",
            ItemKind::MusicDiscPigstep => "music_disc_pigstep",
            ItemKind::Trident => "trident",
            ItemKind::PhantomMembrane => "phantom_membrane",
            ItemKind::NautilusShell => "nautilus_shell",
            ItemKind::HeartOfTheSea => "heart_of_the_sea",
            ItemKind::Crossbow => "crossbow",
            ItemKind::SuspiciousStew => "suspicious_stew",
            ItemKind::Loom => "loom",
            ItemKind::FlowerBannerPattern => "flower_banner_pattern",
            ItemKind::CreeperBannerPattern => "creeper_banner_pattern",
            ItemKind::SkullBannerPattern => "skull_banner_pattern",
            ItemKind::MojangBannerPattern => "mojang_banner_pattern",
            ItemKind::GlobeBannerPattern => "globe_banner_pattern",
            ItemKind::PiglinBannerPattern => "piglin_banner_pattern",
            ItemKind::Composter => "composter",
            ItemKind::Barrel => "barrel",
            ItemKind::Smoker => "smoker",
            ItemKind::BlastFurnace => "blast_furnace",
            ItemKind::CartographyTable => "cartography_table",
            ItemKind::FletchingTable => "fletching_table",
            ItemKind::Grindstone => "grindstone",
            ItemKind::SmithingTable => "smithing_table",
            ItemKind::Stonecutter => "stonecutter",
            ItemKind::Bell => "bell",
            ItemKind::Lantern => "lantern",
            ItemKind::SoulLantern => "soul_lantern",
            ItemKind::SweetBerries => "sweet_berries",
            ItemKind::GlowBerries => "glow_berries",
            ItemKind::Campfire => "campfire",
            ItemKind::SoulCampfire => "soul_campfire",
            ItemKind::Shroomlight => "shroomlight",
            ItemKind::Honeycomb => "honeycomb",
            ItemKind::BeeNest => "bee_nest",
            ItemKind::Beehive => "beehive",
            ItemKind::HoneyBottle => "honey_bottle",
            ItemKind::HoneycombBlock => "honeycomb_block",
            ItemKind::Lodestone => "lodestone",
            ItemKind::CryingObsidian => "crying_obsidian",
            ItemKind::Blackstone => "blackstone",
            ItemKind::BlackstoneSlab => "blackstone_slab",
            ItemKind::BlackstoneStairs => "blackstone_stairs",
            ItemKind::GildedBlackstone => "gilded_blackstone",
            ItemKind::PolishedBlackstone => "polished_blackstone",
            ItemKind::PolishedBlackstoneSlab => "polished_blackstone_slab",
            ItemKind::PolishedBlackstoneStairs => "polished_blackstone_stairs",
            ItemKind::ChiseledPolishedBlackstone => "chiseled_polished_blackstone",
            ItemKind::PolishedBlackstoneBricks => "polished_blackstone_bricks",
            ItemKind::PolishedBlackstoneBrickSlab => "polished_blackstone_brick_slab",
            ItemKind::PolishedBlackstoneBrickStairs => "polished_blackstone_brick_stairs",
            ItemKind::CrackedPolishedBlackstoneBricks => "cracked_polished_blackstone_bricks",
            ItemKind::RespawnAnchor => "respawn_anchor",
            ItemKind::Candle => "candle",
            ItemKind::WhiteCandle => "white_candle",
            ItemKind::OrangeCandle => "orange_candle",
            ItemKind::MagentaCandle => "magenta_candle",
            ItemKind::LightBlueCandle => "light_blue_candle",
            ItemKind::YellowCandle => "yellow_candle",
            ItemKind::LimeCandle => "lime_candle",
            ItemKind::PinkCandle => "pink_candle",
            ItemKind::GrayCandle => "gray_candle",
            ItemKind::LightGrayCandle => "light_gray_candle",
            ItemKind::CyanCandle => "cyan_candle",
            ItemKind::PurpleCandle => "purple_candle",
            ItemKind::BlueCandle => "blue_candle",
            ItemKind::BrownCandle => "brown_candle",
            ItemKind::GreenCandle => "green_candle",
            ItemKind::RedCandle => "red_candle",
            ItemKind::BlackCandle => "black_candle",
            ItemKind::SmallAmethystBud => "small_amethyst_bud",
            ItemKind::MediumAmethystBud => "medium_amethyst_bud",
            ItemKind::LargeAmethystBud => "large_amethyst_bud",
            ItemKind::AmethystCluster => "amethyst_cluster",
            ItemKind::PointedDripstone => "pointed_dripstone",
        }
    }
    #[doc = "Gets a `ItemKind` by its `name`."]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "stone" => Some(ItemKind::Stone),
            "granite" => Some(ItemKind::Granite),
            "polished_granite" => Some(ItemKind::PolishedGranite),
            "diorite" => Some(ItemKind::Diorite),
            "polished_diorite" => Some(ItemKind::PolishedDiorite),
            "andesite" => Some(ItemKind::Andesite),
            "polished_andesite" => Some(ItemKind::PolishedAndesite),
            "deepslate" => Some(ItemKind::Deepslate),
            "cobbled_deepslate" => Some(ItemKind::CobbledDeepslate),
            "polished_deepslate" => Some(ItemKind::PolishedDeepslate),
            "calcite" => Some(ItemKind::Calcite),
            "tuff" => Some(ItemKind::Tuff),
            "dripstone_block" => Some(ItemKind::DripstoneBlock),
            "grass_block" => Some(ItemKind::GrassBlock),
            "dirt" => Some(ItemKind::Dirt),
            "coarse_dirt" => Some(ItemKind::CoarseDirt),
            "podzol" => Some(ItemKind::Podzol),
            "rooted_dirt" => Some(ItemKind::RootedDirt),
            "crimson_nylium" => Some(ItemKind::CrimsonNylium),
            "warped_nylium" => Some(ItemKind::WarpedNylium),
            "cobblestone" => Some(ItemKind::Cobblestone),
            "oak_planks" => Some(ItemKind::OakPlanks),
            "spruce_planks" => Some(ItemKind::SprucePlanks),
            "birch_planks" => Some(ItemKind::BirchPlanks),
            "jungle_planks" => Some(ItemKind::JunglePlanks),
            "acacia_planks" => Some(ItemKind::AcaciaPlanks),
            "dark_oak_planks" => Some(ItemKind::DarkOakPlanks),
            "crimson_planks" => Some(ItemKind::CrimsonPlanks),
            "warped_planks" => Some(ItemKind::WarpedPlanks),
            "oak_sapling" => Some(ItemKind::OakSapling),
            "spruce_sapling" => Some(ItemKind::SpruceSapling),
            "birch_sapling" => Some(ItemKind::BirchSapling),
            "jungle_sapling" => Some(ItemKind::JungleSapling),
            "acacia_sapling" => Some(ItemKind::AcaciaSapling),
            "dark_oak_sapling" => Some(ItemKind::DarkOakSapling),
            "bedrock" => Some(ItemKind::Bedrock),
            "sand" => Some(ItemKind::Sand),
            "red_sand" => Some(ItemKind::RedSand),
            "gravel" => Some(ItemKind::Gravel),
            "coal_ore" => Some(ItemKind::CoalOre),
            "deepslate_coal_ore" => Some(ItemKind::DeepslateCoalOre),
            "iron_ore" => Some(ItemKind::IronOre),
            "deepslate_iron_ore" => Some(ItemKind::DeepslateIronOre),
            "copper_ore" => Some(ItemKind::CopperOre),
            "deepslate_copper_ore" => Some(ItemKind::DeepslateCopperOre),
            "gold_ore" => Some(ItemKind::GoldOre),
            "deepslate_gold_ore" => Some(ItemKind::DeepslateGoldOre),
            "redstone_ore" => Some(ItemKind::RedstoneOre),
            "deepslate_redstone_ore" => Some(ItemKind::DeepslateRedstoneOre),
            "emerald_ore" => Some(ItemKind::EmeraldOre),
            "deepslate_emerald_ore" => Some(ItemKind::DeepslateEmeraldOre),
            "lapis_ore" => Some(ItemKind::LapisOre),
            "deepslate_lapis_ore" => Some(ItemKind::DeepslateLapisOre),
            "diamond_ore" => Some(ItemKind::DiamondOre),
            "deepslate_diamond_ore" => Some(ItemKind::DeepslateDiamondOre),
            "nether_gold_ore" => Some(ItemKind::NetherGoldOre),
            "nether_quartz_ore" => Some(ItemKind::NetherQuartzOre),
            "ancient_debris" => Some(ItemKind::AncientDebris),
            "coal_block" => Some(ItemKind::CoalBlock),
            "raw_iron_block" => Some(ItemKind::RawIronBlock),
            "raw_copper_block" => Some(ItemKind::RawCopperBlock),
            "raw_gold_block" => Some(ItemKind::RawGoldBlock),
            "amethyst_block" => Some(ItemKind::AmethystBlock),
            "budding_amethyst" => Some(ItemKind::BuddingAmethyst),
            "iron_block" => Some(ItemKind::IronBlock),
            "copper_block" => Some(ItemKind::CopperBlock),
            "gold_block" => Some(ItemKind::GoldBlock),
            "diamond_block" => Some(ItemKind::DiamondBlock),
            "netherite_block" => Some(ItemKind::NetheriteBlock),
            "exposed_copper" => Some(ItemKind::ExposedCopper),
            "weathered_copper" => Some(ItemKind::WeatheredCopper),
            "oxidized_copper" => Some(ItemKind::OxidizedCopper),
            "cut_copper" => Some(ItemKind::CutCopper),
            "exposed_cut_copper" => Some(ItemKind::ExposedCutCopper),
            "weathered_cut_copper" => Some(ItemKind::WeatheredCutCopper),
            "oxidized_cut_copper" => Some(ItemKind::OxidizedCutCopper),
            "cut_copper_stairs" => Some(ItemKind::CutCopperStairs),
            "exposed_cut_copper_stairs" => Some(ItemKind::ExposedCutCopperStairs),
            "weathered_cut_copper_stairs" => Some(ItemKind::WeatheredCutCopperStairs),
            "oxidized_cut_copper_stairs" => Some(ItemKind::OxidizedCutCopperStairs),
            "cut_copper_slab" => Some(ItemKind::CutCopperSlab),
            "exposed_cut_copper_slab" => Some(ItemKind::ExposedCutCopperSlab),
            "weathered_cut_copper_slab" => Some(ItemKind::WeatheredCutCopperSlab),
            "oxidized_cut_copper_slab" => Some(ItemKind::OxidizedCutCopperSlab),
            "waxed_copper_block" => Some(ItemKind::WaxedCopperBlock),
            "waxed_exposed_copper" => Some(ItemKind::WaxedExposedCopper),
            "waxed_weathered_copper" => Some(ItemKind::WaxedWeatheredCopper),
            "waxed_oxidized_copper" => Some(ItemKind::WaxedOxidizedCopper),
            "waxed_cut_copper" => Some(ItemKind::WaxedCutCopper),
            "waxed_exposed_cut_copper" => Some(ItemKind::WaxedExposedCutCopper),
            "waxed_weathered_cut_copper" => Some(ItemKind::WaxedWeatheredCutCopper),
            "waxed_oxidized_cut_copper" => Some(ItemKind::WaxedOxidizedCutCopper),
            "waxed_cut_copper_stairs" => Some(ItemKind::WaxedCutCopperStairs),
            "waxed_exposed_cut_copper_stairs" => Some(ItemKind::WaxedExposedCutCopperStairs),
            "waxed_weathered_cut_copper_stairs" => Some(ItemKind::WaxedWeatheredCutCopperStairs),
            "waxed_oxidized_cut_copper_stairs" => Some(ItemKind::WaxedOxidizedCutCopperStairs),
            "waxed_cut_copper_slab" => Some(ItemKind::WaxedCutCopperSlab),
            "waxed_exposed_cut_copper_slab" => Some(ItemKind::WaxedExposedCutCopperSlab),
            "waxed_weathered_cut_copper_slab" => Some(ItemKind::WaxedWeatheredCutCopperSlab),
            "waxed_oxidized_cut_copper_slab" => Some(ItemKind::WaxedOxidizedCutCopperSlab),
            "oak_log" => Some(ItemKind::OakLog),
            "spruce_log" => Some(ItemKind::SpruceLog),
            "birch_log" => Some(ItemKind::BirchLog),
            "jungle_log" => Some(ItemKind::JungleLog),
            "acacia_log" => Some(ItemKind::AcaciaLog),
            "dark_oak_log" => Some(ItemKind::DarkOakLog),
            "crimson_stem" => Some(ItemKind::CrimsonStem),
            "warped_stem" => Some(ItemKind::WarpedStem),
            "stripped_oak_log" => Some(ItemKind::StrippedOakLog),
            "stripped_spruce_log" => Some(ItemKind::StrippedSpruceLog),
            "stripped_birch_log" => Some(ItemKind::StrippedBirchLog),
            "stripped_jungle_log" => Some(ItemKind::StrippedJungleLog),
            "stripped_acacia_log" => Some(ItemKind::StrippedAcaciaLog),
            "stripped_dark_oak_log" => Some(ItemKind::StrippedDarkOakLog),
            "stripped_crimson_stem" => Some(ItemKind::StrippedCrimsonStem),
            "stripped_warped_stem" => Some(ItemKind::StrippedWarpedStem),
            "stripped_oak_wood" => Some(ItemKind::StrippedOakWood),
            "stripped_spruce_wood" => Some(ItemKind::StrippedSpruceWood),
            "stripped_birch_wood" => Some(ItemKind::StrippedBirchWood),
            "stripped_jungle_wood" => Some(ItemKind::StrippedJungleWood),
            "stripped_acacia_wood" => Some(ItemKind::StrippedAcaciaWood),
            "stripped_dark_oak_wood" => Some(ItemKind::StrippedDarkOakWood),
            "stripped_crimson_hyphae" => Some(ItemKind::StrippedCrimsonHyphae),
            "stripped_warped_hyphae" => Some(ItemKind::StrippedWarpedHyphae),
            "oak_wood" => Some(ItemKind::OakWood),
            "spruce_wood" => Some(ItemKind::SpruceWood),
            "birch_wood" => Some(ItemKind::BirchWood),
            "jungle_wood" => Some(ItemKind::JungleWood),
            "acacia_wood" => Some(ItemKind::AcaciaWood),
            "dark_oak_wood" => Some(ItemKind::DarkOakWood),
            "crimson_hyphae" => Some(ItemKind::CrimsonHyphae),
            "warped_hyphae" => Some(ItemKind::WarpedHyphae),
            "oak_leaves" => Some(ItemKind::OakLeaves),
            "spruce_leaves" => Some(ItemKind::SpruceLeaves),
            "birch_leaves" => Some(ItemKind::BirchLeaves),
            "jungle_leaves" => Some(ItemKind::JungleLeaves),
            "acacia_leaves" => Some(ItemKind::AcaciaLeaves),
            "dark_oak_leaves" => Some(ItemKind::DarkOakLeaves),
            "azalea_leaves" => Some(ItemKind::AzaleaLeaves),
            "flowering_azalea_leaves" => Some(ItemKind::FloweringAzaleaLeaves),
            "sponge" => Some(ItemKind::Sponge),
            "wet_sponge" => Some(ItemKind::WetSponge),
            "glass" => Some(ItemKind::Glass),
            "tinted_glass" => Some(ItemKind::TintedGlass),
            "lapis_block" => Some(ItemKind::LapisBlock),
            "sandstone" => Some(ItemKind::Sandstone),
            "chiseled_sandstone" => Some(ItemKind::ChiseledSandstone),
            "cut_sandstone" => Some(ItemKind::CutSandstone),
            "cobweb" => Some(ItemKind::Cobweb),
            "grass" => Some(ItemKind::Grass),
            "fern" => Some(ItemKind::Fern),
            "azalea" => Some(ItemKind::Azalea),
            "flowering_azalea" => Some(ItemKind::FloweringAzalea),
            "dead_bush" => Some(ItemKind::DeadBush),
            "seagrass" => Some(ItemKind::Seagrass),
            "sea_pickle" => Some(ItemKind::SeaPickle),
            "white_wool" => Some(ItemKind::WhiteWool),
            "orange_wool" => Some(ItemKind::OrangeWool),
            "magenta_wool" => Some(ItemKind::MagentaWool),
            "light_blue_wool" => Some(ItemKind::LightBlueWool),
            "yellow_wool" => Some(ItemKind::YellowWool),
            "lime_wool" => Some(ItemKind::LimeWool),
            "pink_wool" => Some(ItemKind::PinkWool),
            "gray_wool" => Some(ItemKind::GrayWool),
            "light_gray_wool" => Some(ItemKind::LightGrayWool),
            "cyan_wool" => Some(ItemKind::CyanWool),
            "purple_wool" => Some(ItemKind::PurpleWool),
            "blue_wool" => Some(ItemKind::BlueWool),
            "brown_wool" => Some(ItemKind::BrownWool),
            "green_wool" => Some(ItemKind::GreenWool),
            "red_wool" => Some(ItemKind::RedWool),
            "black_wool" => Some(ItemKind::BlackWool),
            "dandelion" => Some(ItemKind::Dandelion),
            "poppy" => Some(ItemKind::Poppy),
            "blue_orchid" => Some(ItemKind::BlueOrchid),
            "allium" => Some(ItemKind::Allium),
            "azure_bluet" => Some(ItemKind::AzureBluet),
            "red_tulip" => Some(ItemKind::RedTulip),
            "orange_tulip" => Some(ItemKind::OrangeTulip),
            "white_tulip" => Some(ItemKind::WhiteTulip),
            "pink_tulip" => Some(ItemKind::PinkTulip),
            "oxeye_daisy" => Some(ItemKind::OxeyeDaisy),
            "cornflower" => Some(ItemKind::Cornflower),
            "lily_of_the_valley" => Some(ItemKind::LilyOfTheValley),
            "wither_rose" => Some(ItemKind::WitherRose),
            "spore_blossom" => Some(ItemKind::SporeBlossom),
            "brown_mushroom" => Some(ItemKind::BrownMushroom),
            "red_mushroom" => Some(ItemKind::RedMushroom),
            "crimson_fungus" => Some(ItemKind::CrimsonFungus),
            "warped_fungus" => Some(ItemKind::WarpedFungus),
            "crimson_roots" => Some(ItemKind::CrimsonRoots),
            "warped_roots" => Some(ItemKind::WarpedRoots),
            "nether_sprouts" => Some(ItemKind::NetherSprouts),
            "weeping_vines" => Some(ItemKind::WeepingVines),
            "twisting_vines" => Some(ItemKind::TwistingVines),
            "sugar_cane" => Some(ItemKind::SugarCane),
            "kelp" => Some(ItemKind::Kelp),
            "moss_carpet" => Some(ItemKind::MossCarpet),
            "moss_block" => Some(ItemKind::MossBlock),
            "hanging_roots" => Some(ItemKind::HangingRoots),
            "big_dripleaf" => Some(ItemKind::BigDripleaf),
            "small_dripleaf" => Some(ItemKind::SmallDripleaf),
            "bamboo" => Some(ItemKind::Bamboo),
            "oak_slab" => Some(ItemKind::OakSlab),
            "spruce_slab" => Some(ItemKind::SpruceSlab),
            "birch_slab" => Some(ItemKind::BirchSlab),
            "jungle_slab" => Some(ItemKind::JungleSlab),
            "acacia_slab" => Some(ItemKind::AcaciaSlab),
            "dark_oak_slab" => Some(ItemKind::DarkOakSlab),
            "crimson_slab" => Some(ItemKind::CrimsonSlab),
            "warped_slab" => Some(ItemKind::WarpedSlab),
            "stone_slab" => Some(ItemKind::StoneSlab),
            "smooth_stone_slab" => Some(ItemKind::SmoothStoneSlab),
            "sandstone_slab" => Some(ItemKind::SandstoneSlab),
            "cut_sandstone_slab" => Some(ItemKind::CutSandstoneSlab),
            "petrified_oak_slab" => Some(ItemKind::PetrifiedOakSlab),
            "cobblestone_slab" => Some(ItemKind::CobblestoneSlab),
            "brick_slab" => Some(ItemKind::BrickSlab),
            "stone_brick_slab" => Some(ItemKind::StoneBrickSlab),
            "nether_brick_slab" => Some(ItemKind::NetherBrickSlab),
            "quartz_slab" => Some(ItemKind::QuartzSlab),
            "red_sandstone_slab" => Some(ItemKind::RedSandstoneSlab),
            "cut_red_sandstone_slab" => Some(ItemKind::CutRedSandstoneSlab),
            "purpur_slab" => Some(ItemKind::PurpurSlab),
            "prismarine_slab" => Some(ItemKind::PrismarineSlab),
            "prismarine_brick_slab" => Some(ItemKind::PrismarineBrickSlab),
            "dark_prismarine_slab" => Some(ItemKind::DarkPrismarineSlab),
            "smooth_quartz" => Some(ItemKind::SmoothQuartz),
            "smooth_red_sandstone" => Some(ItemKind::SmoothRedSandstone),
            "smooth_sandstone" => Some(ItemKind::SmoothSandstone),
            "smooth_stone" => Some(ItemKind::SmoothStone),
            "bricks" => Some(ItemKind::Bricks),
            "bookshelf" => Some(ItemKind::Bookshelf),
            "mossy_cobblestone" => Some(ItemKind::MossyCobblestone),
            "obsidian" => Some(ItemKind::Obsidian),
            "torch" => Some(ItemKind::Torch),
            "end_rod" => Some(ItemKind::EndRod),
            "chorus_plant" => Some(ItemKind::ChorusPlant),
            "chorus_flower" => Some(ItemKind::ChorusFlower),
            "purpur_block" => Some(ItemKind::PurpurBlock),
            "purpur_pillar" => Some(ItemKind::PurpurPillar),
            "purpur_stairs" => Some(ItemKind::PurpurStairs),
            "spawner" => Some(ItemKind::Spawner),
            "oak_stairs" => Some(ItemKind::OakStairs),
            "chest" => Some(ItemKind::Chest),
            "crafting_table" => Some(ItemKind::CraftingTable),
            "farmland" => Some(ItemKind::Farmland),
            "furnace" => Some(ItemKind::Furnace),
            "ladder" => Some(ItemKind::Ladder),
            "cobblestone_stairs" => Some(ItemKind::CobblestoneStairs),
            "snow" => Some(ItemKind::Snow),
            "ice" => Some(ItemKind::Ice),
            "snow_block" => Some(ItemKind::SnowBlock),
            "cactus" => Some(ItemKind::Cactus),
            "clay" => Some(ItemKind::Clay),
            "jukebox" => Some(ItemKind::Jukebox),
            "oak_fence" => Some(ItemKind::OakFence),
            "spruce_fence" => Some(ItemKind::SpruceFence),
            "birch_fence" => Some(ItemKind::BirchFence),
            "jungle_fence" => Some(ItemKind::JungleFence),
            "acacia_fence" => Some(ItemKind::AcaciaFence),
            "dark_oak_fence" => Some(ItemKind::DarkOakFence),
            "crimson_fence" => Some(ItemKind::CrimsonFence),
            "warped_fence" => Some(ItemKind::WarpedFence),
            "pumpkin" => Some(ItemKind::Pumpkin),
            "carved_pumpkin" => Some(ItemKind::CarvedPumpkin),
            "jack_o_lantern" => Some(ItemKind::JackOLantern),
            "netherrack" => Some(ItemKind::Netherrack),
            "soul_sand" => Some(ItemKind::SoulSand),
            "soul_soil" => Some(ItemKind::SoulSoil),
            "basalt" => Some(ItemKind::Basalt),
            "polished_basalt" => Some(ItemKind::PolishedBasalt),
            "smooth_basalt" => Some(ItemKind::SmoothBasalt),
            "soul_torch" => Some(ItemKind::SoulTorch),
            "glowstone" => Some(ItemKind::Glowstone),
            "infested_stone" => Some(ItemKind::InfestedStone),
            "infested_cobblestone" => Some(ItemKind::InfestedCobblestone),
            "infested_stone_bricks" => Some(ItemKind::InfestedStoneBricks),
            "infested_mossy_stone_bricks" => Some(ItemKind::InfestedMossyStoneBricks),
            "infested_cracked_stone_bricks" => Some(ItemKind::InfestedCrackedStoneBricks),
            "infested_chiseled_stone_bricks" => Some(ItemKind::InfestedChiseledStoneBricks),
            "infested_deepslate" => Some(ItemKind::InfestedDeepslate),
            "stone_bricks" => Some(ItemKind::StoneBricks),
            "mossy_stone_bricks" => Some(ItemKind::MossyStoneBricks),
            "cracked_stone_bricks" => Some(ItemKind::CrackedStoneBricks),
            "chiseled_stone_bricks" => Some(ItemKind::ChiseledStoneBricks),
            "deepslate_bricks" => Some(ItemKind::DeepslateBricks),
            "cracked_deepslate_bricks" => Some(ItemKind::CrackedDeepslateBricks),
            "deepslate_tiles" => Some(ItemKind::DeepslateTiles),
            "cracked_deepslate_tiles" => Some(ItemKind::CrackedDeepslateTiles),
            "chiseled_deepslate" => Some(ItemKind::ChiseledDeepslate),
            "brown_mushroom_block" => Some(ItemKind::BrownMushroomBlock),
            "red_mushroom_block" => Some(ItemKind::RedMushroomBlock),
            "mushroom_stem" => Some(ItemKind::MushroomStem),
            "iron_bars" => Some(ItemKind::IronBars),
            "chain" => Some(ItemKind::Chain),
            "glass_pane" => Some(ItemKind::GlassPane),
            "melon" => Some(ItemKind::Melon),
            "vine" => Some(ItemKind::Vine),
            "glow_lichen" => Some(ItemKind::GlowLichen),
            "brick_stairs" => Some(ItemKind::BrickStairs),
            "stone_brick_stairs" => Some(ItemKind::StoneBrickStairs),
            "mycelium" => Some(ItemKind::Mycelium),
            "lily_pad" => Some(ItemKind::LilyPad),
            "nether_bricks" => Some(ItemKind::NetherBricks),
            "cracked_nether_bricks" => Some(ItemKind::CrackedNetherBricks),
            "chiseled_nether_bricks" => Some(ItemKind::ChiseledNetherBricks),
            "nether_brick_fence" => Some(ItemKind::NetherBrickFence),
            "nether_brick_stairs" => Some(ItemKind::NetherBrickStairs),
            "enchanting_table" => Some(ItemKind::EnchantingTable),
            "end_portal_frame" => Some(ItemKind::EndPortalFrame),
            "end_stone" => Some(ItemKind::EndStone),
            "end_stone_bricks" => Some(ItemKind::EndStoneBricks),
            "dragon_egg" => Some(ItemKind::DragonEgg),
            "sandstone_stairs" => Some(ItemKind::SandstoneStairs),
            "ender_chest" => Some(ItemKind::EnderChest),
            "emerald_block" => Some(ItemKind::EmeraldBlock),
            "spruce_stairs" => Some(ItemKind::SpruceStairs),
            "birch_stairs" => Some(ItemKind::BirchStairs),
            "jungle_stairs" => Some(ItemKind::JungleStairs),
            "crimson_stairs" => Some(ItemKind::CrimsonStairs),
            "warped_stairs" => Some(ItemKind::WarpedStairs),
            "command_block" => Some(ItemKind::CommandBlock),
            "beacon" => Some(ItemKind::Beacon),
            "cobblestone_wall" => Some(ItemKind::CobblestoneWall),
            "mossy_cobblestone_wall" => Some(ItemKind::MossyCobblestoneWall),
            "brick_wall" => Some(ItemKind::BrickWall),
            "prismarine_wall" => Some(ItemKind::PrismarineWall),
            "red_sandstone_wall" => Some(ItemKind::RedSandstoneWall),
            "mossy_stone_brick_wall" => Some(ItemKind::MossyStoneBrickWall),
            "granite_wall" => Some(ItemKind::GraniteWall),
            "stone_brick_wall" => Some(ItemKind::StoneBrickWall),
            "nether_brick_wall" => Some(ItemKind::NetherBrickWall),
            "andesite_wall" => Some(ItemKind::AndesiteWall),
            "red_nether_brick_wall" => Some(ItemKind::RedNetherBrickWall),
            "sandstone_wall" => Some(ItemKind::SandstoneWall),
            "end_stone_brick_wall" => Some(ItemKind::EndStoneBrickWall),
            "diorite_wall" => Some(ItemKind::DioriteWall),
            "blackstone_wall" => Some(ItemKind::BlackstoneWall),
            "polished_blackstone_wall" => Some(ItemKind::PolishedBlackstoneWall),
            "polished_blackstone_brick_wall" => Some(ItemKind::PolishedBlackstoneBrickWall),
            "cobbled_deepslate_wall" => Some(ItemKind::CobbledDeepslateWall),
            "polished_deepslate_wall" => Some(ItemKind::PolishedDeepslateWall),
            "deepslate_brick_wall" => Some(ItemKind::DeepslateBrickWall),
            "deepslate_tile_wall" => Some(ItemKind::DeepslateTileWall),
            "anvil" => Some(ItemKind::Anvil),
            "chipped_anvil" => Some(ItemKind::ChippedAnvil),
            "damaged_anvil" => Some(ItemKind::DamagedAnvil),
            "chiseled_quartz_block" => Some(ItemKind::ChiseledQuartzBlock),
            "quartz_block" => Some(ItemKind::QuartzBlock),
            "quartz_bricks" => Some(ItemKind::QuartzBricks),
            "quartz_pillar" => Some(ItemKind::QuartzPillar),
            "quartz_stairs" => Some(ItemKind::QuartzStairs),
            "white_terracotta" => Some(ItemKind::WhiteTerracotta),
            "orange_terracotta" => Some(ItemKind::OrangeTerracotta),
            "magenta_terracotta" => Some(ItemKind::MagentaTerracotta),
            "light_blue_terracotta" => Some(ItemKind::LightBlueTerracotta),
            "yellow_terracotta" => Some(ItemKind::YellowTerracotta),
            "lime_terracotta" => Some(ItemKind::LimeTerracotta),
            "pink_terracotta" => Some(ItemKind::PinkTerracotta),
            "gray_terracotta" => Some(ItemKind::GrayTerracotta),
            "light_gray_terracotta" => Some(ItemKind::LightGrayTerracotta),
            "cyan_terracotta" => Some(ItemKind::CyanTerracotta),
            "purple_terracotta" => Some(ItemKind::PurpleTerracotta),
            "blue_terracotta" => Some(ItemKind::BlueTerracotta),
            "brown_terracotta" => Some(ItemKind::BrownTerracotta),
            "green_terracotta" => Some(ItemKind::GreenTerracotta),
            "red_terracotta" => Some(ItemKind::RedTerracotta),
            "black_terracotta" => Some(ItemKind::BlackTerracotta),
            "barrier" => Some(ItemKind::Barrier),
            "light" => Some(ItemKind::Light),
            "hay_block" => Some(ItemKind::HayBlock),
            "white_carpet" => Some(ItemKind::WhiteCarpet),
            "orange_carpet" => Some(ItemKind::OrangeCarpet),
            "magenta_carpet" => Some(ItemKind::MagentaCarpet),
            "light_blue_carpet" => Some(ItemKind::LightBlueCarpet),
            "yellow_carpet" => Some(ItemKind::YellowCarpet),
            "lime_carpet" => Some(ItemKind::LimeCarpet),
            "pink_carpet" => Some(ItemKind::PinkCarpet),
            "gray_carpet" => Some(ItemKind::GrayCarpet),
            "light_gray_carpet" => Some(ItemKind::LightGrayCarpet),
            "cyan_carpet" => Some(ItemKind::CyanCarpet),
            "purple_carpet" => Some(ItemKind::PurpleCarpet),
            "blue_carpet" => Some(ItemKind::BlueCarpet),
            "brown_carpet" => Some(ItemKind::BrownCarpet),
            "green_carpet" => Some(ItemKind::GreenCarpet),
            "red_carpet" => Some(ItemKind::RedCarpet),
            "black_carpet" => Some(ItemKind::BlackCarpet),
            "terracotta" => Some(ItemKind::Terracotta),
            "packed_ice" => Some(ItemKind::PackedIce),
            "acacia_stairs" => Some(ItemKind::AcaciaStairs),
            "dark_oak_stairs" => Some(ItemKind::DarkOakStairs),
            "dirt_path" => Some(ItemKind::DirtPath),
            "sunflower" => Some(ItemKind::Sunflower),
            "lilac" => Some(ItemKind::Lilac),
            "rose_bush" => Some(ItemKind::RoseBush),
            "peony" => Some(ItemKind::Peony),
            "tall_grass" => Some(ItemKind::TallGrass),
            "large_fern" => Some(ItemKind::LargeFern),
            "white_stained_glass" => Some(ItemKind::WhiteStainedGlass),
            "orange_stained_glass" => Some(ItemKind::OrangeStainedGlass),
            "magenta_stained_glass" => Some(ItemKind::MagentaStainedGlass),
            "light_blue_stained_glass" => Some(ItemKind::LightBlueStainedGlass),
            "yellow_stained_glass" => Some(ItemKind::YellowStainedGlass),
            "lime_stained_glass" => Some(ItemKind::LimeStainedGlass),
            "pink_stained_glass" => Some(ItemKind::PinkStainedGlass),
            "gray_stained_glass" => Some(ItemKind::GrayStainedGlass),
            "light_gray_stained_glass" => Some(ItemKind::LightGrayStainedGlass),
            "cyan_stained_glass" => Some(ItemKind::CyanStainedGlass),
            "purple_stained_glass" => Some(ItemKind::PurpleStainedGlass),
            "blue_stained_glass" => Some(ItemKind::BlueStainedGlass),
            "brown_stained_glass" => Some(ItemKind::BrownStainedGlass),
            "green_stained_glass" => Some(ItemKind::GreenStainedGlass),
            "red_stained_glass" => Some(ItemKind::RedStainedGlass),
            "black_stained_glass" => Some(ItemKind::BlackStainedGlass),
            "white_stained_glass_pane" => Some(ItemKind::WhiteStainedGlassPane),
            "orange_stained_glass_pane" => Some(ItemKind::OrangeStainedGlassPane),
            "magenta_stained_glass_pane" => Some(ItemKind::MagentaStainedGlassPane),
            "light_blue_stained_glass_pane" => Some(ItemKind::LightBlueStainedGlassPane),
            "yellow_stained_glass_pane" => Some(ItemKind::YellowStainedGlassPane),
            "lime_stained_glass_pane" => Some(ItemKind::LimeStainedGlassPane),
            "pink_stained_glass_pane" => Some(ItemKind::PinkStainedGlassPane),
            "gray_stained_glass_pane" => Some(ItemKind::GrayStainedGlassPane),
            "light_gray_stained_glass_pane" => Some(ItemKind::LightGrayStainedGlassPane),
            "cyan_stained_glass_pane" => Some(ItemKind::CyanStainedGlassPane),
            "purple_stained_glass_pane" => Some(ItemKind::PurpleStainedGlassPane),
            "blue_stained_glass_pane" => Some(ItemKind::BlueStainedGlassPane),
            "brown_stained_glass_pane" => Some(ItemKind::BrownStainedGlassPane),
            "green_stained_glass_pane" => Some(ItemKind::GreenStainedGlassPane),
            "red_stained_glass_pane" => Some(ItemKind::RedStainedGlassPane),
            "black_stained_glass_pane" => Some(ItemKind::BlackStainedGlassPane),
            "prismarine" => Some(ItemKind::Prismarine),
            "prismarine_bricks" => Some(ItemKind::PrismarineBricks),
            "dark_prismarine" => Some(ItemKind::DarkPrismarine),
            "prismarine_stairs" => Some(ItemKind::PrismarineStairs),
            "prismarine_brick_stairs" => Some(ItemKind::PrismarineBrickStairs),
            "dark_prismarine_stairs" => Some(ItemKind::DarkPrismarineStairs),
            "sea_lantern" => Some(ItemKind::SeaLantern),
            "red_sandstone" => Some(ItemKind::RedSandstone),
            "chiseled_red_sandstone" => Some(ItemKind::ChiseledRedSandstone),
            "cut_red_sandstone" => Some(ItemKind::CutRedSandstone),
            "red_sandstone_stairs" => Some(ItemKind::RedSandstoneStairs),
            "repeating_command_block" => Some(ItemKind::RepeatingCommandBlock),
            "chain_command_block" => Some(ItemKind::ChainCommandBlock),
            "magma_block" => Some(ItemKind::MagmaBlock),
            "nether_wart_block" => Some(ItemKind::NetherWartBlock),
            "warped_wart_block" => Some(ItemKind::WarpedWartBlock),
            "red_nether_bricks" => Some(ItemKind::RedNetherBricks),
            "bone_block" => Some(ItemKind::BoneBlock),
            "structure_void" => Some(ItemKind::StructureVoid),
            "shulker_box" => Some(ItemKind::ShulkerBox),
            "white_shulker_box" => Some(ItemKind::WhiteShulkerBox),
            "orange_shulker_box" => Some(ItemKind::OrangeShulkerBox),
            "magenta_shulker_box" => Some(ItemKind::MagentaShulkerBox),
            "light_blue_shulker_box" => Some(ItemKind::LightBlueShulkerBox),
            "yellow_shulker_box" => Some(ItemKind::YellowShulkerBox),
            "lime_shulker_box" => Some(ItemKind::LimeShulkerBox),
            "pink_shulker_box" => Some(ItemKind::PinkShulkerBox),
            "gray_shulker_box" => Some(ItemKind::GrayShulkerBox),
            "light_gray_shulker_box" => Some(ItemKind::LightGrayShulkerBox),
            "cyan_shulker_box" => Some(ItemKind::CyanShulkerBox),
            "purple_shulker_box" => Some(ItemKind::PurpleShulkerBox),
            "blue_shulker_box" => Some(ItemKind::BlueShulkerBox),
            "brown_shulker_box" => Some(ItemKind::BrownShulkerBox),
            "green_shulker_box" => Some(ItemKind::GreenShulkerBox),
            "red_shulker_box" => Some(ItemKind::RedShulkerBox),
            "black_shulker_box" => Some(ItemKind::BlackShulkerBox),
            "white_glazed_terracotta" => Some(ItemKind::WhiteGlazedTerracotta),
            "orange_glazed_terracotta" => Some(ItemKind::OrangeGlazedTerracotta),
            "magenta_glazed_terracotta" => Some(ItemKind::MagentaGlazedTerracotta),
            "light_blue_glazed_terracotta" => Some(ItemKind::LightBlueGlazedTerracotta),
            "yellow_glazed_terracotta" => Some(ItemKind::YellowGlazedTerracotta),
            "lime_glazed_terracotta" => Some(ItemKind::LimeGlazedTerracotta),
            "pink_glazed_terracotta" => Some(ItemKind::PinkGlazedTerracotta),
            "gray_glazed_terracotta" => Some(ItemKind::GrayGlazedTerracotta),
            "light_gray_glazed_terracotta" => Some(ItemKind::LightGrayGlazedTerracotta),
            "cyan_glazed_terracotta" => Some(ItemKind::CyanGlazedTerracotta),
            "purple_glazed_terracotta" => Some(ItemKind::PurpleGlazedTerracotta),
            "blue_glazed_terracotta" => Some(ItemKind::BlueGlazedTerracotta),
            "brown_glazed_terracotta" => Some(ItemKind::BrownGlazedTerracotta),
            "green_glazed_terracotta" => Some(ItemKind::GreenGlazedTerracotta),
            "red_glazed_terracotta" => Some(ItemKind::RedGlazedTerracotta),
            "black_glazed_terracotta" => Some(ItemKind::BlackGlazedTerracotta),
            "white_concrete" => Some(ItemKind::WhiteConcrete),
            "orange_concrete" => Some(ItemKind::OrangeConcrete),
            "magenta_concrete" => Some(ItemKind::MagentaConcrete),
            "light_blue_concrete" => Some(ItemKind::LightBlueConcrete),
            "yellow_concrete" => Some(ItemKind::YellowConcrete),
            "lime_concrete" => Some(ItemKind::LimeConcrete),
            "pink_concrete" => Some(ItemKind::PinkConcrete),
            "gray_concrete" => Some(ItemKind::GrayConcrete),
            "light_gray_concrete" => Some(ItemKind::LightGrayConcrete),
            "cyan_concrete" => Some(ItemKind::CyanConcrete),
            "purple_concrete" => Some(ItemKind::PurpleConcrete),
            "blue_concrete" => Some(ItemKind::BlueConcrete),
            "brown_concrete" => Some(ItemKind::BrownConcrete),
            "green_concrete" => Some(ItemKind::GreenConcrete),
            "red_concrete" => Some(ItemKind::RedConcrete),
            "black_concrete" => Some(ItemKind::BlackConcrete),
            "white_concrete_powder" => Some(ItemKind::WhiteConcretePowder),
            "orange_concrete_powder" => Some(ItemKind::OrangeConcretePowder),
            "magenta_concrete_powder" => Some(ItemKind::MagentaConcretePowder),
            "light_blue_concrete_powder" => Some(ItemKind::LightBlueConcretePowder),
            "yellow_concrete_powder" => Some(ItemKind::YellowConcretePowder),
            "lime_concrete_powder" => Some(ItemKind::LimeConcretePowder),
            "pink_concrete_powder" => Some(ItemKind::PinkConcretePowder),
            "gray_concrete_powder" => Some(ItemKind::GrayConcretePowder),
            "light_gray_concrete_powder" => Some(ItemKind::LightGrayConcretePowder),
            "cyan_concrete_powder" => Some(ItemKind::CyanConcretePowder),
            "purple_concrete_powder" => Some(ItemKind::PurpleConcretePowder),
            "blue_concrete_powder" => Some(ItemKind::BlueConcretePowder),
            "brown_concrete_powder" => Some(ItemKind::BrownConcretePowder),
            "green_concrete_powder" => Some(ItemKind::GreenConcretePowder),
            "red_concrete_powder" => Some(ItemKind::RedConcretePowder),
            "black_concrete_powder" => Some(ItemKind::BlackConcretePowder),
            "turtle_egg" => Some(ItemKind::TurtleEgg),
            "dead_tube_coral_block" => Some(ItemKind::DeadTubeCoralBlock),
            "dead_brain_coral_block" => Some(ItemKind::DeadBrainCoralBlock),
            "dead_bubble_coral_block" => Some(ItemKind::DeadBubbleCoralBlock),
            "dead_fire_coral_block" => Some(ItemKind::DeadFireCoralBlock),
            "dead_horn_coral_block" => Some(ItemKind::DeadHornCoralBlock),
            "tube_coral_block" => Some(ItemKind::TubeCoralBlock),
            "brain_coral_block" => Some(ItemKind::BrainCoralBlock),
            "bubble_coral_block" => Some(ItemKind::BubbleCoralBlock),
            "fire_coral_block" => Some(ItemKind::FireCoralBlock),
            "horn_coral_block" => Some(ItemKind::HornCoralBlock),
            "tube_coral" => Some(ItemKind::TubeCoral),
            "brain_coral" => Some(ItemKind::BrainCoral),
            "bubble_coral" => Some(ItemKind::BubbleCoral),
            "fire_coral" => Some(ItemKind::FireCoral),
            "horn_coral" => Some(ItemKind::HornCoral),
            "dead_brain_coral" => Some(ItemKind::DeadBrainCoral),
            "dead_bubble_coral" => Some(ItemKind::DeadBubbleCoral),
            "dead_fire_coral" => Some(ItemKind::DeadFireCoral),
            "dead_horn_coral" => Some(ItemKind::DeadHornCoral),
            "dead_tube_coral" => Some(ItemKind::DeadTubeCoral),
            "tube_coral_fan" => Some(ItemKind::TubeCoralFan),
            "brain_coral_fan" => Some(ItemKind::BrainCoralFan),
            "bubble_coral_fan" => Some(ItemKind::BubbleCoralFan),
            "fire_coral_fan" => Some(ItemKind::FireCoralFan),
            "horn_coral_fan" => Some(ItemKind::HornCoralFan),
            "dead_tube_coral_fan" => Some(ItemKind::DeadTubeCoralFan),
            "dead_brain_coral_fan" => Some(ItemKind::DeadBrainCoralFan),
            "dead_bubble_coral_fan" => Some(ItemKind::DeadBubbleCoralFan),
            "dead_fire_coral_fan" => Some(ItemKind::DeadFireCoralFan),
            "dead_horn_coral_fan" => Some(ItemKind::DeadHornCoralFan),
            "blue_ice" => Some(ItemKind::BlueIce),
            "conduit" => Some(ItemKind::Conduit),
            "polished_granite_stairs" => Some(ItemKind::PolishedGraniteStairs),
            "smooth_red_sandstone_stairs" => Some(ItemKind::SmoothRedSandstoneStairs),
            "mossy_stone_brick_stairs" => Some(ItemKind::MossyStoneBrickStairs),
            "polished_diorite_stairs" => Some(ItemKind::PolishedDioriteStairs),
            "mossy_cobblestone_stairs" => Some(ItemKind::MossyCobblestoneStairs),
            "end_stone_brick_stairs" => Some(ItemKind::EndStoneBrickStairs),
            "stone_stairs" => Some(ItemKind::StoneStairs),
            "smooth_sandstone_stairs" => Some(ItemKind::SmoothSandstoneStairs),
            "smooth_quartz_stairs" => Some(ItemKind::SmoothQuartzStairs),
            "granite_stairs" => Some(ItemKind::GraniteStairs),
            "andesite_stairs" => Some(ItemKind::AndesiteStairs),
            "red_nether_brick_stairs" => Some(ItemKind::RedNetherBrickStairs),
            "polished_andesite_stairs" => Some(ItemKind::PolishedAndesiteStairs),
            "diorite_stairs" => Some(ItemKind::DioriteStairs),
            "cobbled_deepslate_stairs" => Some(ItemKind::CobbledDeepslateStairs),
            "polished_deepslate_stairs" => Some(ItemKind::PolishedDeepslateStairs),
            "deepslate_brick_stairs" => Some(ItemKind::DeepslateBrickStairs),
            "deepslate_tile_stairs" => Some(ItemKind::DeepslateTileStairs),
            "polished_granite_slab" => Some(ItemKind::PolishedGraniteSlab),
            "smooth_red_sandstone_slab" => Some(ItemKind::SmoothRedSandstoneSlab),
            "mossy_stone_brick_slab" => Some(ItemKind::MossyStoneBrickSlab),
            "polished_diorite_slab" => Some(ItemKind::PolishedDioriteSlab),
            "mossy_cobblestone_slab" => Some(ItemKind::MossyCobblestoneSlab),
            "end_stone_brick_slab" => Some(ItemKind::EndStoneBrickSlab),
            "smooth_sandstone_slab" => Some(ItemKind::SmoothSandstoneSlab),
            "smooth_quartz_slab" => Some(ItemKind::SmoothQuartzSlab),
            "granite_slab" => Some(ItemKind::GraniteSlab),
            "andesite_slab" => Some(ItemKind::AndesiteSlab),
            "red_nether_brick_slab" => Some(ItemKind::RedNetherBrickSlab),
            "polished_andesite_slab" => Some(ItemKind::PolishedAndesiteSlab),
            "diorite_slab" => Some(ItemKind::DioriteSlab),
            "cobbled_deepslate_slab" => Some(ItemKind::CobbledDeepslateSlab),
            "polished_deepslate_slab" => Some(ItemKind::PolishedDeepslateSlab),
            "deepslate_brick_slab" => Some(ItemKind::DeepslateBrickSlab),
            "deepslate_tile_slab" => Some(ItemKind::DeepslateTileSlab),
            "scaffolding" => Some(ItemKind::Scaffolding),
            "redstone" => Some(ItemKind::Redstone),
            "redstone_torch" => Some(ItemKind::RedstoneTorch),
            "redstone_block" => Some(ItemKind::RedstoneBlock),
            "repeater" => Some(ItemKind::Repeater),
            "comparator" => Some(ItemKind::Comparator),
            "piston" => Some(ItemKind::Piston),
            "sticky_piston" => Some(ItemKind::StickyPiston),
            "slime_block" => Some(ItemKind::SlimeBlock),
            "honey_block" => Some(ItemKind::HoneyBlock),
            "observer" => Some(ItemKind::Observer),
            "hopper" => Some(ItemKind::Hopper),
            "dispenser" => Some(ItemKind::Dispenser),
            "dropper" => Some(ItemKind::Dropper),
            "lectern" => Some(ItemKind::Lectern),
            "target" => Some(ItemKind::Target),
            "lever" => Some(ItemKind::Lever),
            "lightning_rod" => Some(ItemKind::LightningRod),
            "daylight_detector" => Some(ItemKind::DaylightDetector),
            "sculk_sensor" => Some(ItemKind::SculkSensor),
            "tripwire_hook" => Some(ItemKind::TripwireHook),
            "trapped_chest" => Some(ItemKind::TrappedChest),
            "tnt" => Some(ItemKind::Tnt),
            "redstone_lamp" => Some(ItemKind::RedstoneLamp),
            "note_block" => Some(ItemKind::NoteBlock),
            "stone_button" => Some(ItemKind::StoneButton),
            "polished_blackstone_button" => Some(ItemKind::PolishedBlackstoneButton),
            "oak_button" => Some(ItemKind::OakButton),
            "spruce_button" => Some(ItemKind::SpruceButton),
            "birch_button" => Some(ItemKind::BirchButton),
            "jungle_button" => Some(ItemKind::JungleButton),
            "acacia_button" => Some(ItemKind::AcaciaButton),
            "dark_oak_button" => Some(ItemKind::DarkOakButton),
            "crimson_button" => Some(ItemKind::CrimsonButton),
            "warped_button" => Some(ItemKind::WarpedButton),
            "stone_pressure_plate" => Some(ItemKind::StonePressurePlate),
            "polished_blackstone_pressure_plate" => Some(ItemKind::PolishedBlackstonePressurePlate),
            "light_weighted_pressure_plate" => Some(ItemKind::LightWeightedPressurePlate),
            "heavy_weighted_pressure_plate" => Some(ItemKind::HeavyWeightedPressurePlate),
            "oak_pressure_plate" => Some(ItemKind::OakPressurePlate),
            "spruce_pressure_plate" => Some(ItemKind::SprucePressurePlate),
            "birch_pressure_plate" => Some(ItemKind::BirchPressurePlate),
            "jungle_pressure_plate" => Some(ItemKind::JunglePressurePlate),
            "acacia_pressure_plate" => Some(ItemKind::AcaciaPressurePlate),
            "dark_oak_pressure_plate" => Some(ItemKind::DarkOakPressurePlate),
            "crimson_pressure_plate" => Some(ItemKind::CrimsonPressurePlate),
            "warped_pressure_plate" => Some(ItemKind::WarpedPressurePlate),
            "iron_door" => Some(ItemKind::IronDoor),
            "oak_door" => Some(ItemKind::OakDoor),
            "spruce_door" => Some(ItemKind::SpruceDoor),
            "birch_door" => Some(ItemKind::BirchDoor),
            "jungle_door" => Some(ItemKind::JungleDoor),
            "acacia_door" => Some(ItemKind::AcaciaDoor),
            "dark_oak_door" => Some(ItemKind::DarkOakDoor),
            "crimson_door" => Some(ItemKind::CrimsonDoor),
            "warped_door" => Some(ItemKind::WarpedDoor),
            "iron_trapdoor" => Some(ItemKind::IronTrapdoor),
            "oak_trapdoor" => Some(ItemKind::OakTrapdoor),
            "spruce_trapdoor" => Some(ItemKind::SpruceTrapdoor),
            "birch_trapdoor" => Some(ItemKind::BirchTrapdoor),
            "jungle_trapdoor" => Some(ItemKind::JungleTrapdoor),
            "acacia_trapdoor" => Some(ItemKind::AcaciaTrapdoor),
            "dark_oak_trapdoor" => Some(ItemKind::DarkOakTrapdoor),
            "crimson_trapdoor" => Some(ItemKind::CrimsonTrapdoor),
            "warped_trapdoor" => Some(ItemKind::WarpedTrapdoor),
            "oak_fence_gate" => Some(ItemKind::OakFenceGate),
            "spruce_fence_gate" => Some(ItemKind::SpruceFenceGate),
            "birch_fence_gate" => Some(ItemKind::BirchFenceGate),
            "jungle_fence_gate" => Some(ItemKind::JungleFenceGate),
            "acacia_fence_gate" => Some(ItemKind::AcaciaFenceGate),
            "dark_oak_fence_gate" => Some(ItemKind::DarkOakFenceGate),
            "crimson_fence_gate" => Some(ItemKind::CrimsonFenceGate),
            "warped_fence_gate" => Some(ItemKind::WarpedFenceGate),
            "powered_rail" => Some(ItemKind::PoweredRail),
            "detector_rail" => Some(ItemKind::DetectorRail),
            "rail" => Some(ItemKind::Rail),
            "activator_rail" => Some(ItemKind::ActivatorRail),
            "saddle" => Some(ItemKind::Saddle),
            "minecart" => Some(ItemKind::Minecart),
            "chest_minecart" => Some(ItemKind::ChestMinecart),
            "furnace_minecart" => Some(ItemKind::FurnaceMinecart),
            "tnt_minecart" => Some(ItemKind::TntMinecart),
            "hopper_minecart" => Some(ItemKind::HopperMinecart),
            "carrot_on_a_stick" => Some(ItemKind::CarrotOnAStick),
            "warped_fungus_on_a_stick" => Some(ItemKind::WarpedFungusOnAStick),
            "elytra" => Some(ItemKind::Elytra),
            "oak_boat" => Some(ItemKind::OakBoat),
            "spruce_boat" => Some(ItemKind::SpruceBoat),
            "birch_boat" => Some(ItemKind::BirchBoat),
            "jungle_boat" => Some(ItemKind::JungleBoat),
            "acacia_boat" => Some(ItemKind::AcaciaBoat),
            "dark_oak_boat" => Some(ItemKind::DarkOakBoat),
            "structure_block" => Some(ItemKind::StructureBlock),
            "jigsaw" => Some(ItemKind::Jigsaw),
            "turtle_helmet" => Some(ItemKind::TurtleHelmet),
            "scute" => Some(ItemKind::Scute),
            "flint_and_steel" => Some(ItemKind::FlintAndSteel),
            "apple" => Some(ItemKind::Apple),
            "bow" => Some(ItemKind::Bow),
            "arrow" => Some(ItemKind::Arrow),
            "coal" => Some(ItemKind::Coal),
            "charcoal" => Some(ItemKind::Charcoal),
            "diamond" => Some(ItemKind::Diamond),
            "emerald" => Some(ItemKind::Emerald),
            "lapis_lazuli" => Some(ItemKind::LapisLazuli),
            "quartz" => Some(ItemKind::Quartz),
            "amethyst_shard" => Some(ItemKind::AmethystShard),
            "raw_iron" => Some(ItemKind::RawIron),
            "iron_ingot" => Some(ItemKind::IronIngot),
            "raw_copper" => Some(ItemKind::RawCopper),
            "copper_ingot" => Some(ItemKind::CopperIngot),
            "raw_gold" => Some(ItemKind::RawGold),
            "gold_ingot" => Some(ItemKind::GoldIngot),
            "netherite_ingot" => Some(ItemKind::NetheriteIngot),
            "netherite_scrap" => Some(ItemKind::NetheriteScrap),
            "wooden_sword" => Some(ItemKind::WoodenSword),
            "wooden_shovel" => Some(ItemKind::WoodenShovel),
            "wooden_pickaxe" => Some(ItemKind::WoodenPickaxe),
            "wooden_axe" => Some(ItemKind::WoodenAxe),
            "wooden_hoe" => Some(ItemKind::WoodenHoe),
            "stone_sword" => Some(ItemKind::StoneSword),
            "stone_shovel" => Some(ItemKind::StoneShovel),
            "stone_pickaxe" => Some(ItemKind::StonePickaxe),
            "stone_axe" => Some(ItemKind::StoneAxe),
            "stone_hoe" => Some(ItemKind::StoneHoe),
            "golden_sword" => Some(ItemKind::GoldenSword),
            "golden_shovel" => Some(ItemKind::GoldenShovel),
            "golden_pickaxe" => Some(ItemKind::GoldenPickaxe),
            "golden_axe" => Some(ItemKind::GoldenAxe),
            "golden_hoe" => Some(ItemKind::GoldenHoe),
            "iron_sword" => Some(ItemKind::IronSword),
            "iron_shovel" => Some(ItemKind::IronShovel),
            "iron_pickaxe" => Some(ItemKind::IronPickaxe),
            "iron_axe" => Some(ItemKind::IronAxe),
            "iron_hoe" => Some(ItemKind::IronHoe),
            "diamond_sword" => Some(ItemKind::DiamondSword),
            "diamond_shovel" => Some(ItemKind::DiamondShovel),
            "diamond_pickaxe" => Some(ItemKind::DiamondPickaxe),
            "diamond_axe" => Some(ItemKind::DiamondAxe),
            "diamond_hoe" => Some(ItemKind::DiamondHoe),
            "netherite_sword" => Some(ItemKind::NetheriteSword),
            "netherite_shovel" => Some(ItemKind::NetheriteShovel),
            "netherite_pickaxe" => Some(ItemKind::NetheritePickaxe),
            "netherite_axe" => Some(ItemKind::NetheriteAxe),
            "netherite_hoe" => Some(ItemKind::NetheriteHoe),
            "stick" => Some(ItemKind::Stick),
            "bowl" => Some(ItemKind::Bowl),
            "mushroom_stew" => Some(ItemKind::MushroomStew),
            "string" => Some(ItemKind::String),
            "feather" => Some(ItemKind::Feather),
            "gunpowder" => Some(ItemKind::Gunpowder),
            "wheat_seeds" => Some(ItemKind::WheatSeeds),
            "wheat" => Some(ItemKind::Wheat),
            "bread" => Some(ItemKind::Bread),
            "leather_helmet" => Some(ItemKind::LeatherHelmet),
            "leather_chestplate" => Some(ItemKind::LeatherChestplate),
            "leather_leggings" => Some(ItemKind::LeatherLeggings),
            "leather_boots" => Some(ItemKind::LeatherBoots),
            "chainmail_helmet" => Some(ItemKind::ChainmailHelmet),
            "chainmail_chestplate" => Some(ItemKind::ChainmailChestplate),
            "chainmail_leggings" => Some(ItemKind::ChainmailLeggings),
            "chainmail_boots" => Some(ItemKind::ChainmailBoots),
            "iron_helmet" => Some(ItemKind::IronHelmet),
            "iron_chestplate" => Some(ItemKind::IronChestplate),
            "iron_leggings" => Some(ItemKind::IronLeggings),
            "iron_boots" => Some(ItemKind::IronBoots),
            "diamond_helmet" => Some(ItemKind::DiamondHelmet),
            "diamond_chestplate" => Some(ItemKind::DiamondChestplate),
            "diamond_leggings" => Some(ItemKind::DiamondLeggings),
            "diamond_boots" => Some(ItemKind::DiamondBoots),
            "golden_helmet" => Some(ItemKind::GoldenHelmet),
            "golden_chestplate" => Some(ItemKind::GoldenChestplate),
            "golden_leggings" => Some(ItemKind::GoldenLeggings),
            "golden_boots" => Some(ItemKind::GoldenBoots),
            "netherite_helmet" => Some(ItemKind::NetheriteHelmet),
            "netherite_chestplate" => Some(ItemKind::NetheriteChestplate),
            "netherite_leggings" => Some(ItemKind::NetheriteLeggings),
            "netherite_boots" => Some(ItemKind::NetheriteBoots),
            "flint" => Some(ItemKind::Flint),
            "porkchop" => Some(ItemKind::Porkchop),
            "cooked_porkchop" => Some(ItemKind::CookedPorkchop),
            "painting" => Some(ItemKind::Painting),
            "golden_apple" => Some(ItemKind::GoldenApple),
            "enchanted_golden_apple" => Some(ItemKind::EnchantedGoldenApple),
            "oak_sign" => Some(ItemKind::OakSign),
            "spruce_sign" => Some(ItemKind::SpruceSign),
            "birch_sign" => Some(ItemKind::BirchSign),
            "jungle_sign" => Some(ItemKind::JungleSign),
            "acacia_sign" => Some(ItemKind::AcaciaSign),
            "dark_oak_sign" => Some(ItemKind::DarkOakSign),
            "crimson_sign" => Some(ItemKind::CrimsonSign),
            "warped_sign" => Some(ItemKind::WarpedSign),
            "bucket" => Some(ItemKind::Bucket),
            "water_bucket" => Some(ItemKind::WaterBucket),
            "lava_bucket" => Some(ItemKind::LavaBucket),
            "powder_snow_bucket" => Some(ItemKind::PowderSnowBucket),
            "snowball" => Some(ItemKind::Snowball),
            "leather" => Some(ItemKind::Leather),
            "milk_bucket" => Some(ItemKind::MilkBucket),
            "pufferfish_bucket" => Some(ItemKind::PufferfishBucket),
            "salmon_bucket" => Some(ItemKind::SalmonBucket),
            "cod_bucket" => Some(ItemKind::CodBucket),
            "tropical_fish_bucket" => Some(ItemKind::TropicalFishBucket),
            "axolotl_bucket" => Some(ItemKind::AxolotlBucket),
            "brick" => Some(ItemKind::Brick),
            "clay_ball" => Some(ItemKind::ClayBall),
            "dried_kelp_block" => Some(ItemKind::DriedKelpBlock),
            "paper" => Some(ItemKind::Paper),
            "book" => Some(ItemKind::Book),
            "slime_ball" => Some(ItemKind::SlimeBall),
            "egg" => Some(ItemKind::Egg),
            "compass" => Some(ItemKind::Compass),
            "bundle" => Some(ItemKind::Bundle),
            "fishing_rod" => Some(ItemKind::FishingRod),
            "clock" => Some(ItemKind::Clock),
            "spyglass" => Some(ItemKind::Spyglass),
            "glowstone_dust" => Some(ItemKind::GlowstoneDust),
            "cod" => Some(ItemKind::Cod),
            "salmon" => Some(ItemKind::Salmon),
            "tropical_fish" => Some(ItemKind::TropicalFish),
            "pufferfish" => Some(ItemKind::Pufferfish),
            "cooked_cod" => Some(ItemKind::CookedCod),
            "cooked_salmon" => Some(ItemKind::CookedSalmon),
            "ink_sac" => Some(ItemKind::InkSac),
            "glow_ink_sac" => Some(ItemKind::GlowInkSac),
            "cocoa_beans" => Some(ItemKind::CocoaBeans),
            "white_dye" => Some(ItemKind::WhiteDye),
            "orange_dye" => Some(ItemKind::OrangeDye),
            "magenta_dye" => Some(ItemKind::MagentaDye),
            "light_blue_dye" => Some(ItemKind::LightBlueDye),
            "yellow_dye" => Some(ItemKind::YellowDye),
            "lime_dye" => Some(ItemKind::LimeDye),
            "pink_dye" => Some(ItemKind::PinkDye),
            "gray_dye" => Some(ItemKind::GrayDye),
            "light_gray_dye" => Some(ItemKind::LightGrayDye),
            "cyan_dye" => Some(ItemKind::CyanDye),
            "purple_dye" => Some(ItemKind::PurpleDye),
            "blue_dye" => Some(ItemKind::BlueDye),
            "brown_dye" => Some(ItemKind::BrownDye),
            "green_dye" => Some(ItemKind::GreenDye),
            "red_dye" => Some(ItemKind::RedDye),
            "black_dye" => Some(ItemKind::BlackDye),
            "bone_meal" => Some(ItemKind::BoneMeal),
            "bone" => Some(ItemKind::Bone),
            "sugar" => Some(ItemKind::Sugar),
            "cake" => Some(ItemKind::Cake),
            "white_bed" => Some(ItemKind::WhiteBed),
            "orange_bed" => Some(ItemKind::OrangeBed),
            "magenta_bed" => Some(ItemKind::MagentaBed),
            "light_blue_bed" => Some(ItemKind::LightBlueBed),
            "yellow_bed" => Some(ItemKind::YellowBed),
            "lime_bed" => Some(ItemKind::LimeBed),
            "pink_bed" => Some(ItemKind::PinkBed),
            "gray_bed" => Some(ItemKind::GrayBed),
            "light_gray_bed" => Some(ItemKind::LightGrayBed),
            "cyan_bed" => Some(ItemKind::CyanBed),
            "purple_bed" => Some(ItemKind::PurpleBed),
            "blue_bed" => Some(ItemKind::BlueBed),
            "brown_bed" => Some(ItemKind::BrownBed),
            "green_bed" => Some(ItemKind::GreenBed),
            "red_bed" => Some(ItemKind::RedBed),
            "black_bed" => Some(ItemKind::BlackBed),
            "cookie" => Some(ItemKind::Cookie),
            "filled_map" => Some(ItemKind::FilledMap),
            "shears" => Some(ItemKind::Shears),
            "melon_slice" => Some(ItemKind::MelonSlice),
            "dried_kelp" => Some(ItemKind::DriedKelp),
            "pumpkin_seeds" => Some(ItemKind::PumpkinSeeds),
            "melon_seeds" => Some(ItemKind::MelonSeeds),
            "beef" => Some(ItemKind::Beef),
            "cooked_beef" => Some(ItemKind::CookedBeef),
            "chicken" => Some(ItemKind::Chicken),
            "cooked_chicken" => Some(ItemKind::CookedChicken),
            "rotten_flesh" => Some(ItemKind::RottenFlesh),
            "ender_pearl" => Some(ItemKind::EnderPearl),
            "blaze_rod" => Some(ItemKind::BlazeRod),
            "ghast_tear" => Some(ItemKind::GhastTear),
            "gold_nugget" => Some(ItemKind::GoldNugget),
            "nether_wart" => Some(ItemKind::NetherWart),
            "potion" => Some(ItemKind::Potion),
            "glass_bottle" => Some(ItemKind::GlassBottle),
            "spider_eye" => Some(ItemKind::SpiderEye),
            "fermented_spider_eye" => Some(ItemKind::FermentedSpiderEye),
            "blaze_powder" => Some(ItemKind::BlazePowder),
            "magma_cream" => Some(ItemKind::MagmaCream),
            "brewing_stand" => Some(ItemKind::BrewingStand),
            "cauldron" => Some(ItemKind::Cauldron),
            "ender_eye" => Some(ItemKind::EnderEye),
            "glistering_melon_slice" => Some(ItemKind::GlisteringMelonSlice),
            "axolotl_spawn_egg" => Some(ItemKind::AxolotlSpawnEgg),
            "bat_spawn_egg" => Some(ItemKind::BatSpawnEgg),
            "bee_spawn_egg" => Some(ItemKind::BeeSpawnEgg),
            "blaze_spawn_egg" => Some(ItemKind::BlazeSpawnEgg),
            "cat_spawn_egg" => Some(ItemKind::CatSpawnEgg),
            "cave_spider_spawn_egg" => Some(ItemKind::CaveSpiderSpawnEgg),
            "chicken_spawn_egg" => Some(ItemKind::ChickenSpawnEgg),
            "cod_spawn_egg" => Some(ItemKind::CodSpawnEgg),
            "cow_spawn_egg" => Some(ItemKind::CowSpawnEgg),
            "creeper_spawn_egg" => Some(ItemKind::CreeperSpawnEgg),
            "dolphin_spawn_egg" => Some(ItemKind::DolphinSpawnEgg),
            "donkey_spawn_egg" => Some(ItemKind::DonkeySpawnEgg),
            "drowned_spawn_egg" => Some(ItemKind::DrownedSpawnEgg),
            "elder_guardian_spawn_egg" => Some(ItemKind::ElderGuardianSpawnEgg),
            "enderman_spawn_egg" => Some(ItemKind::EndermanSpawnEgg),
            "endermite_spawn_egg" => Some(ItemKind::EndermiteSpawnEgg),
            "evoker_spawn_egg" => Some(ItemKind::EvokerSpawnEgg),
            "fox_spawn_egg" => Some(ItemKind::FoxSpawnEgg),
            "ghast_spawn_egg" => Some(ItemKind::GhastSpawnEgg),
            "glow_squid_spawn_egg" => Some(ItemKind::GlowSquidSpawnEgg),
            "goat_spawn_egg" => Some(ItemKind::GoatSpawnEgg),
            "guardian_spawn_egg" => Some(ItemKind::GuardianSpawnEgg),
            "hoglin_spawn_egg" => Some(ItemKind::HoglinSpawnEgg),
            "horse_spawn_egg" => Some(ItemKind::HorseSpawnEgg),
            "husk_spawn_egg" => Some(ItemKind::HuskSpawnEgg),
            "llama_spawn_egg" => Some(ItemKind::LlamaSpawnEgg),
            "magma_cube_spawn_egg" => Some(ItemKind::MagmaCubeSpawnEgg),
            "mooshroom_spawn_egg" => Some(ItemKind::MooshroomSpawnEgg),
            "mule_spawn_egg" => Some(ItemKind::MuleSpawnEgg),
            "ocelot_spawn_egg" => Some(ItemKind::OcelotSpawnEgg),
            "panda_spawn_egg" => Some(ItemKind::PandaSpawnEgg),
            "parrot_spawn_egg" => Some(ItemKind::ParrotSpawnEgg),
            "phantom_spawn_egg" => Some(ItemKind::PhantomSpawnEgg),
            "pig_spawn_egg" => Some(ItemKind::PigSpawnEgg),
            "piglin_spawn_egg" => Some(ItemKind::PiglinSpawnEgg),
            "piglin_brute_spawn_egg" => Some(ItemKind::PiglinBruteSpawnEgg),
            "pillager_spawn_egg" => Some(ItemKind::PillagerSpawnEgg),
            "polar_bear_spawn_egg" => Some(ItemKind::PolarBearSpawnEgg),
            "pufferfish_spawn_egg" => Some(ItemKind::PufferfishSpawnEgg),
            "rabbit_spawn_egg" => Some(ItemKind::RabbitSpawnEgg),
            "ravager_spawn_egg" => Some(ItemKind::RavagerSpawnEgg),
            "salmon_spawn_egg" => Some(ItemKind::SalmonSpawnEgg),
            "sheep_spawn_egg" => Some(ItemKind::SheepSpawnEgg),
            "shulker_spawn_egg" => Some(ItemKind::ShulkerSpawnEgg),
            "silverfish_spawn_egg" => Some(ItemKind::SilverfishSpawnEgg),
            "skeleton_spawn_egg" => Some(ItemKind::SkeletonSpawnEgg),
            "skeleton_horse_spawn_egg" => Some(ItemKind::SkeletonHorseSpawnEgg),
            "slime_spawn_egg" => Some(ItemKind::SlimeSpawnEgg),
            "spider_spawn_egg" => Some(ItemKind::SpiderSpawnEgg),
            "squid_spawn_egg" => Some(ItemKind::SquidSpawnEgg),
            "stray_spawn_egg" => Some(ItemKind::StraySpawnEgg),
            "strider_spawn_egg" => Some(ItemKind::StriderSpawnEgg),
            "trader_llama_spawn_egg" => Some(ItemKind::TraderLlamaSpawnEgg),
            "tropical_fish_spawn_egg" => Some(ItemKind::TropicalFishSpawnEgg),
            "turtle_spawn_egg" => Some(ItemKind::TurtleSpawnEgg),
            "vex_spawn_egg" => Some(ItemKind::VexSpawnEgg),
            "villager_spawn_egg" => Some(ItemKind::VillagerSpawnEgg),
            "vindicator_spawn_egg" => Some(ItemKind::VindicatorSpawnEgg),
            "wandering_trader_spawn_egg" => Some(ItemKind::WanderingTraderSpawnEgg),
            "witch_spawn_egg" => Some(ItemKind::WitchSpawnEgg),
            "wither_skeleton_spawn_egg" => Some(ItemKind::WitherSkeletonSpawnEgg),
            "wolf_spawn_egg" => Some(ItemKind::WolfSpawnEgg),
            "zoglin_spawn_egg" => Some(ItemKind::ZoglinSpawnEgg),
            "zombie_spawn_egg" => Some(ItemKind::ZombieSpawnEgg),
            "zombie_horse_spawn_egg" => Some(ItemKind::ZombieHorseSpawnEgg),
            "zombie_villager_spawn_egg" => Some(ItemKind::ZombieVillagerSpawnEgg),
            "zombified_piglin_spawn_egg" => Some(ItemKind::ZombifiedPiglinSpawnEgg),
            "experience_bottle" => Some(ItemKind::ExperienceBottle),
            "fire_charge" => Some(ItemKind::FireCharge),
            "writable_book" => Some(ItemKind::WritableBook),
            "written_book" => Some(ItemKind::WrittenBook),
            "item_frame" => Some(ItemKind::ItemFrame),
            "glow_item_frame" => Some(ItemKind::GlowItemFrame),
            "flower_pot" => Some(ItemKind::FlowerPot),
            "carrot" => Some(ItemKind::Carrot),
            "potato" => Some(ItemKind::Potato),
            "baked_potato" => Some(ItemKind::BakedPotato),
            "poisonous_potato" => Some(ItemKind::PoisonousPotato),
            "map" => Some(ItemKind::Map),
            "golden_carrot" => Some(ItemKind::GoldenCarrot),
            "skeleton_skull" => Some(ItemKind::SkeletonSkull),
            "wither_skeleton_skull" => Some(ItemKind::WitherSkeletonSkull),
            "player_head" => Some(ItemKind::PlayerHead),
            "zombie_head" => Some(ItemKind::ZombieHead),
            "creeper_head" => Some(ItemKind::CreeperHead),
            "dragon_head" => Some(ItemKind::DragonHead),
            "nether_star" => Some(ItemKind::NetherStar),
            "pumpkin_pie" => Some(ItemKind::PumpkinPie),
            "firework_rocket" => Some(ItemKind::FireworkRocket),
            "firework_star" => Some(ItemKind::FireworkStar),
            "enchanted_book" => Some(ItemKind::EnchantedBook),
            "nether_brick" => Some(ItemKind::NetherBrick),
            "prismarine_shard" => Some(ItemKind::PrismarineShard),
            "prismarine_crystals" => Some(ItemKind::PrismarineCrystals),
            "rabbit" => Some(ItemKind::Rabbit),
            "cooked_rabbit" => Some(ItemKind::CookedRabbit),
            "rabbit_stew" => Some(ItemKind::RabbitStew),
            "rabbit_foot" => Some(ItemKind::RabbitFoot),
            "rabbit_hide" => Some(ItemKind::RabbitHide),
            "armor_stand" => Some(ItemKind::ArmorStand),
            "iron_horse_armor" => Some(ItemKind::IronHorseArmor),
            "golden_horse_armor" => Some(ItemKind::GoldenHorseArmor),
            "diamond_horse_armor" => Some(ItemKind::DiamondHorseArmor),
            "leather_horse_armor" => Some(ItemKind::LeatherHorseArmor),
            "lead" => Some(ItemKind::Lead),
            "name_tag" => Some(ItemKind::NameTag),
            "command_block_minecart" => Some(ItemKind::CommandBlockMinecart),
            "mutton" => Some(ItemKind::Mutton),
            "cooked_mutton" => Some(ItemKind::CookedMutton),
            "white_banner" => Some(ItemKind::WhiteBanner),
            "orange_banner" => Some(ItemKind::OrangeBanner),
            "magenta_banner" => Some(ItemKind::MagentaBanner),
            "light_blue_banner" => Some(ItemKind::LightBlueBanner),
            "yellow_banner" => Some(ItemKind::YellowBanner),
            "lime_banner" => Some(ItemKind::LimeBanner),
            "pink_banner" => Some(ItemKind::PinkBanner),
            "gray_banner" => Some(ItemKind::GrayBanner),
            "light_gray_banner" => Some(ItemKind::LightGrayBanner),
            "cyan_banner" => Some(ItemKind::CyanBanner),
            "purple_banner" => Some(ItemKind::PurpleBanner),
            "blue_banner" => Some(ItemKind::BlueBanner),
            "brown_banner" => Some(ItemKind::BrownBanner),
            "green_banner" => Some(ItemKind::GreenBanner),
            "red_banner" => Some(ItemKind::RedBanner),
            "black_banner" => Some(ItemKind::BlackBanner),
            "end_crystal" => Some(ItemKind::EndCrystal),
            "chorus_fruit" => Some(ItemKind::ChorusFruit),
            "popped_chorus_fruit" => Some(ItemKind::PoppedChorusFruit),
            "beetroot" => Some(ItemKind::Beetroot),
            "beetroot_seeds" => Some(ItemKind::BeetrootSeeds),
            "beetroot_soup" => Some(ItemKind::BeetrootSoup),
            "dragon_breath" => Some(ItemKind::DragonBreath),
            "splash_potion" => Some(ItemKind::SplashPotion),
            "spectral_arrow" => Some(ItemKind::SpectralArrow),
            "tipped_arrow" => Some(ItemKind::TippedArrow),
            "lingering_potion" => Some(ItemKind::LingeringPotion),
            "shield" => Some(ItemKind::Shield),
            "totem_of_undying" => Some(ItemKind::TotemOfUndying),
            "shulker_shell" => Some(ItemKind::ShulkerShell),
            "iron_nugget" => Some(ItemKind::IronNugget),
            "knowledge_book" => Some(ItemKind::KnowledgeBook),
            "debug_stick" => Some(ItemKind::DebugStick),
            "music_disc_13" => Some(ItemKind::MusicDisc13),
            "music_disc_cat" => Some(ItemKind::MusicDiscCat),
            "music_disc_blocks" => Some(ItemKind::MusicDiscBlocks),
            "music_disc_chirp" => Some(ItemKind::MusicDiscChirp),
            "music_disc_far" => Some(ItemKind::MusicDiscFar),
            "music_disc_mall" => Some(ItemKind::MusicDiscMall),
            "music_disc_mellohi" => Some(ItemKind::MusicDiscMellohi),
            "music_disc_stal" => Some(ItemKind::MusicDiscStal),
            "music_disc_strad" => Some(ItemKind::MusicDiscStrad),
            "music_disc_ward" => Some(ItemKind::MusicDiscWard),
            "music_disc_11" => Some(ItemKind::MusicDisc11),
            "music_disc_wait" => Some(ItemKind::MusicDiscWait),
            "music_disc_otherside" => Some(ItemKind::MusicDiscOtherside),
            "music_disc_pigstep" => Some(ItemKind::MusicDiscPigstep),
            "trident" => Some(ItemKind::Trident),
            "phantom_membrane" => Some(ItemKind::PhantomMembrane),
            "nautilus_shell" => Some(ItemKind::NautilusShell),
            "heart_of_the_sea" => Some(ItemKind::HeartOfTheSea),
            "crossbow" => Some(ItemKind::Crossbow),
            "suspicious_stew" => Some(ItemKind::SuspiciousStew),
            "loom" => Some(ItemKind::Loom),
            "flower_banner_pattern" => Some(ItemKind::FlowerBannerPattern),
            "creeper_banner_pattern" => Some(ItemKind::CreeperBannerPattern),
            "skull_banner_pattern" => Some(ItemKind::SkullBannerPattern),
            "mojang_banner_pattern" => Some(ItemKind::MojangBannerPattern),
            "globe_banner_pattern" => Some(ItemKind::GlobeBannerPattern),
            "piglin_banner_pattern" => Some(ItemKind::PiglinBannerPattern),
            "composter" => Some(ItemKind::Composter),
            "barrel" => Some(ItemKind::Barrel),
            "smoker" => Some(ItemKind::Smoker),
            "blast_furnace" => Some(ItemKind::BlastFurnace),
            "cartography_table" => Some(ItemKind::CartographyTable),
            "fletching_table" => Some(ItemKind::FletchingTable),
            "grindstone" => Some(ItemKind::Grindstone),
            "smithing_table" => Some(ItemKind::SmithingTable),
            "stonecutter" => Some(ItemKind::Stonecutter),
            "bell" => Some(ItemKind::Bell),
            "lantern" => Some(ItemKind::Lantern),
            "soul_lantern" => Some(ItemKind::SoulLantern),
            "sweet_berries" => Some(ItemKind::SweetBerries),
            "glow_berries" => Some(ItemKind::GlowBerries),
            "campfire" => Some(ItemKind::Campfire),
            "soul_campfire" => Some(ItemKind::SoulCampfire),
            "shroomlight" => Some(ItemKind::Shroomlight),
            "honeycomb" => Some(ItemKind::Honeycomb),
            "bee_nest" => Some(ItemKind::BeeNest),
            "beehive" => Some(ItemKind::Beehive),
            "honey_bottle" => Some(ItemKind::HoneyBottle),
            "honeycomb_block" => Some(ItemKind::HoneycombBlock),
            "lodestone" => Some(ItemKind::Lodestone),
            "crying_obsidian" => Some(ItemKind::CryingObsidian),
            "blackstone" => Some(ItemKind::Blackstone),
            "blackstone_slab" => Some(ItemKind::BlackstoneSlab),
            "blackstone_stairs" => Some(ItemKind::BlackstoneStairs),
            "gilded_blackstone" => Some(ItemKind::GildedBlackstone),
            "polished_blackstone" => Some(ItemKind::PolishedBlackstone),
            "polished_blackstone_slab" => Some(ItemKind::PolishedBlackstoneSlab),
            "polished_blackstone_stairs" => Some(ItemKind::PolishedBlackstoneStairs),
            "chiseled_polished_blackstone" => Some(ItemKind::ChiseledPolishedBlackstone),
            "polished_blackstone_bricks" => Some(ItemKind::PolishedBlackstoneBricks),
            "polished_blackstone_brick_slab" => Some(ItemKind::PolishedBlackstoneBrickSlab),
            "polished_blackstone_brick_stairs" => Some(ItemKind::PolishedBlackstoneBrickStairs),
            "cracked_polished_blackstone_bricks" => Some(ItemKind::CrackedPolishedBlackstoneBricks),
            "respawn_anchor" => Some(ItemKind::RespawnAnchor),
            "candle" => Some(ItemKind::Candle),
            "white_candle" => Some(ItemKind::WhiteCandle),
            "orange_candle" => Some(ItemKind::OrangeCandle),
            "magenta_candle" => Some(ItemKind::MagentaCandle),
            "light_blue_candle" => Some(ItemKind::LightBlueCandle),
            "yellow_candle" => Some(ItemKind::YellowCandle),
            "lime_candle" => Some(ItemKind::LimeCandle),
            "pink_candle" => Some(ItemKind::PinkCandle),
            "gray_candle" => Some(ItemKind::GrayCandle),
            "light_gray_candle" => Some(ItemKind::LightGrayCandle),
            "cyan_candle" => Some(ItemKind::CyanCandle),
            "purple_candle" => Some(ItemKind::PurpleCandle),
            "blue_candle" => Some(ItemKind::BlueCandle),
            "brown_candle" => Some(ItemKind::BrownCandle),
            "green_candle" => Some(ItemKind::GreenCandle),
            "red_candle" => Some(ItemKind::RedCandle),
            "black_candle" => Some(ItemKind::BlackCandle),
            "small_amethyst_bud" => Some(ItemKind::SmallAmethystBud),
            "medium_amethyst_bud" => Some(ItemKind::MediumAmethystBud),
            "large_amethyst_bud" => Some(ItemKind::LargeAmethystBud),
            "amethyst_cluster" => Some(ItemKind::AmethystCluster),
            "pointed_dripstone" => Some(ItemKind::PointedDripstone),
            _ => None,
        }
    }
}
impl ItemKind {
    #[doc = "Returns the `namespaced_id` property of this `ItemKind`."]
    #[inline]
    pub const fn namespaced_id(&self) -> &'static str {
        match self {
            ItemKind::Stone => "minecraft:stone",
            ItemKind::Granite => "minecraft:granite",
            ItemKind::PolishedGranite => "minecraft:polished_granite",
            ItemKind::Diorite => "minecraft:diorite",
            ItemKind::PolishedDiorite => "minecraft:polished_diorite",
            ItemKind::Andesite => "minecraft:andesite",
            ItemKind::PolishedAndesite => "minecraft:polished_andesite",
            ItemKind::Deepslate => "minecraft:deepslate",
            ItemKind::CobbledDeepslate => "minecraft:cobbled_deepslate",
            ItemKind::PolishedDeepslate => "minecraft:polished_deepslate",
            ItemKind::Calcite => "minecraft:calcite",
            ItemKind::Tuff => "minecraft:tuff",
            ItemKind::DripstoneBlock => "minecraft:dripstone_block",
            ItemKind::GrassBlock => "minecraft:grass_block",
            ItemKind::Dirt => "minecraft:dirt",
            ItemKind::CoarseDirt => "minecraft:coarse_dirt",
            ItemKind::Podzol => "minecraft:podzol",
            ItemKind::RootedDirt => "minecraft:rooted_dirt",
            ItemKind::CrimsonNylium => "minecraft:crimson_nylium",
            ItemKind::WarpedNylium => "minecraft:warped_nylium",
            ItemKind::Cobblestone => "minecraft:cobblestone",
            ItemKind::OakPlanks => "minecraft:oak_planks",
            ItemKind::SprucePlanks => "minecraft:spruce_planks",
            ItemKind::BirchPlanks => "minecraft:birch_planks",
            ItemKind::JunglePlanks => "minecraft:jungle_planks",
            ItemKind::AcaciaPlanks => "minecraft:acacia_planks",
            ItemKind::DarkOakPlanks => "minecraft:dark_oak_planks",
            ItemKind::CrimsonPlanks => "minecraft:crimson_planks",
            ItemKind::WarpedPlanks => "minecraft:warped_planks",
            ItemKind::OakSapling => "minecraft:oak_sapling",
            ItemKind::SpruceSapling => "minecraft:spruce_sapling",
            ItemKind::BirchSapling => "minecraft:birch_sapling",
            ItemKind::JungleSapling => "minecraft:jungle_sapling",
            ItemKind::AcaciaSapling => "minecraft:acacia_sapling",
            ItemKind::DarkOakSapling => "minecraft:dark_oak_sapling",
            ItemKind::Bedrock => "minecraft:bedrock",
            ItemKind::Sand => "minecraft:sand",
            ItemKind::RedSand => "minecraft:red_sand",
            ItemKind::Gravel => "minecraft:gravel",
            ItemKind::CoalOre => "minecraft:coal_ore",
            ItemKind::DeepslateCoalOre => "minecraft:deepslate_coal_ore",
            ItemKind::IronOre => "minecraft:iron_ore",
            ItemKind::DeepslateIronOre => "minecraft:deepslate_iron_ore",
            ItemKind::CopperOre => "minecraft:copper_ore",
            ItemKind::DeepslateCopperOre => "minecraft:deepslate_copper_ore",
            ItemKind::GoldOre => "minecraft:gold_ore",
            ItemKind::DeepslateGoldOre => "minecraft:deepslate_gold_ore",
            ItemKind::RedstoneOre => "minecraft:redstone_ore",
            ItemKind::DeepslateRedstoneOre => "minecraft:deepslate_redstone_ore",
            ItemKind::EmeraldOre => "minecraft:emerald_ore",
            ItemKind::DeepslateEmeraldOre => "minecraft:deepslate_emerald_ore",
            ItemKind::LapisOre => "minecraft:lapis_ore",
            ItemKind::DeepslateLapisOre => "minecraft:deepslate_lapis_ore",
            ItemKind::DiamondOre => "minecraft:diamond_ore",
            ItemKind::DeepslateDiamondOre => "minecraft:deepslate_diamond_ore",
            ItemKind::NetherGoldOre => "minecraft:nether_gold_ore",
            ItemKind::NetherQuartzOre => "minecraft:nether_quartz_ore",
            ItemKind::AncientDebris => "minecraft:ancient_debris",
            ItemKind::CoalBlock => "minecraft:coal_block",
            ItemKind::RawIronBlock => "minecraft:raw_iron_block",
            ItemKind::RawCopperBlock => "minecraft:raw_copper_block",
            ItemKind::RawGoldBlock => "minecraft:raw_gold_block",
            ItemKind::AmethystBlock => "minecraft:amethyst_block",
            ItemKind::BuddingAmethyst => "minecraft:budding_amethyst",
            ItemKind::IronBlock => "minecraft:iron_block",
            ItemKind::CopperBlock => "minecraft:copper_block",
            ItemKind::GoldBlock => "minecraft:gold_block",
            ItemKind::DiamondBlock => "minecraft:diamond_block",
            ItemKind::NetheriteBlock => "minecraft:netherite_block",
            ItemKind::ExposedCopper => "minecraft:exposed_copper",
            ItemKind::WeatheredCopper => "minecraft:weathered_copper",
            ItemKind::OxidizedCopper => "minecraft:oxidized_copper",
            ItemKind::CutCopper => "minecraft:cut_copper",
            ItemKind::ExposedCutCopper => "minecraft:exposed_cut_copper",
            ItemKind::WeatheredCutCopper => "minecraft:weathered_cut_copper",
            ItemKind::OxidizedCutCopper => "minecraft:oxidized_cut_copper",
            ItemKind::CutCopperStairs => "minecraft:cut_copper_stairs",
            ItemKind::ExposedCutCopperStairs => "minecraft:exposed_cut_copper_stairs",
            ItemKind::WeatheredCutCopperStairs => "minecraft:weathered_cut_copper_stairs",
            ItemKind::OxidizedCutCopperStairs => "minecraft:oxidized_cut_copper_stairs",
            ItemKind::CutCopperSlab => "minecraft:cut_copper_slab",
            ItemKind::ExposedCutCopperSlab => "minecraft:exposed_cut_copper_slab",
            ItemKind::WeatheredCutCopperSlab => "minecraft:weathered_cut_copper_slab",
            ItemKind::OxidizedCutCopperSlab => "minecraft:oxidized_cut_copper_slab",
            ItemKind::WaxedCopperBlock => "minecraft:waxed_copper_block",
            ItemKind::WaxedExposedCopper => "minecraft:waxed_exposed_copper",
            ItemKind::WaxedWeatheredCopper => "minecraft:waxed_weathered_copper",
            ItemKind::WaxedOxidizedCopper => "minecraft:waxed_oxidized_copper",
            ItemKind::WaxedCutCopper => "minecraft:waxed_cut_copper",
            ItemKind::WaxedExposedCutCopper => "minecraft:waxed_exposed_cut_copper",
            ItemKind::WaxedWeatheredCutCopper => "minecraft:waxed_weathered_cut_copper",
            ItemKind::WaxedOxidizedCutCopper => "minecraft:waxed_oxidized_cut_copper",
            ItemKind::WaxedCutCopperStairs => "minecraft:waxed_cut_copper_stairs",
            ItemKind::WaxedExposedCutCopperStairs => "minecraft:waxed_exposed_cut_copper_stairs",
            ItemKind::WaxedWeatheredCutCopperStairs => {
                "minecraft:waxed_weathered_cut_copper_stairs"
            }
            ItemKind::WaxedOxidizedCutCopperStairs => "minecraft:waxed_oxidized_cut_copper_stairs",
            ItemKind::WaxedCutCopperSlab => "minecraft:waxed_cut_copper_slab",
            ItemKind::WaxedExposedCutCopperSlab => "minecraft:waxed_exposed_cut_copper_slab",
            ItemKind::WaxedWeatheredCutCopperSlab => "minecraft:waxed_weathered_cut_copper_slab",
            ItemKind::WaxedOxidizedCutCopperSlab => "minecraft:waxed_oxidized_cut_copper_slab",
            ItemKind::OakLog => "minecraft:oak_log",
            ItemKind::SpruceLog => "minecraft:spruce_log",
            ItemKind::BirchLog => "minecraft:birch_log",
            ItemKind::JungleLog => "minecraft:jungle_log",
            ItemKind::AcaciaLog => "minecraft:acacia_log",
            ItemKind::DarkOakLog => "minecraft:dark_oak_log",
            ItemKind::CrimsonStem => "minecraft:crimson_stem",
            ItemKind::WarpedStem => "minecraft:warped_stem",
            ItemKind::StrippedOakLog => "minecraft:stripped_oak_log",
            ItemKind::StrippedSpruceLog => "minecraft:stripped_spruce_log",
            ItemKind::StrippedBirchLog => "minecraft:stripped_birch_log",
            ItemKind::StrippedJungleLog => "minecraft:stripped_jungle_log",
            ItemKind::StrippedAcaciaLog => "minecraft:stripped_acacia_log",
            ItemKind::StrippedDarkOakLog => "minecraft:stripped_dark_oak_log",
            ItemKind::StrippedCrimsonStem => "minecraft:stripped_crimson_stem",
            ItemKind::StrippedWarpedStem => "minecraft:stripped_warped_stem",
            ItemKind::StrippedOakWood => "minecraft:stripped_oak_wood",
            ItemKind::StrippedSpruceWood => "minecraft:stripped_spruce_wood",
            ItemKind::StrippedBirchWood => "minecraft:stripped_birch_wood",
            ItemKind::StrippedJungleWood => "minecraft:stripped_jungle_wood",
            ItemKind::StrippedAcaciaWood => "minecraft:stripped_acacia_wood",
            ItemKind::StrippedDarkOakWood => "minecraft:stripped_dark_oak_wood",
            ItemKind::StrippedCrimsonHyphae => "minecraft:stripped_crimson_hyphae",
            ItemKind::StrippedWarpedHyphae => "minecraft:stripped_warped_hyphae",
            ItemKind::OakWood => "minecraft:oak_wood",
            ItemKind::SpruceWood => "minecraft:spruce_wood",
            ItemKind::BirchWood => "minecraft:birch_wood",
            ItemKind::JungleWood => "minecraft:jungle_wood",
            ItemKind::AcaciaWood => "minecraft:acacia_wood",
            ItemKind::DarkOakWood => "minecraft:dark_oak_wood",
            ItemKind::CrimsonHyphae => "minecraft:crimson_hyphae",
            ItemKind::WarpedHyphae => "minecraft:warped_hyphae",
            ItemKind::OakLeaves => "minecraft:oak_leaves",
            ItemKind::SpruceLeaves => "minecraft:spruce_leaves",
            ItemKind::BirchLeaves => "minecraft:birch_leaves",
            ItemKind::JungleLeaves => "minecraft:jungle_leaves",
            ItemKind::AcaciaLeaves => "minecraft:acacia_leaves",
            ItemKind::DarkOakLeaves => "minecraft:dark_oak_leaves",
            ItemKind::AzaleaLeaves => "minecraft:azalea_leaves",
            ItemKind::FloweringAzaleaLeaves => "minecraft:flowering_azalea_leaves",
            ItemKind::Sponge => "minecraft:sponge",
            ItemKind::WetSponge => "minecraft:wet_sponge",
            ItemKind::Glass => "minecraft:glass",
            ItemKind::TintedGlass => "minecraft:tinted_glass",
            ItemKind::LapisBlock => "minecraft:lapis_block",
            ItemKind::Sandstone => "minecraft:sandstone",
            ItemKind::ChiseledSandstone => "minecraft:chiseled_sandstone",
            ItemKind::CutSandstone => "minecraft:cut_sandstone",
            ItemKind::Cobweb => "minecraft:cobweb",
            ItemKind::Grass => "minecraft:grass",
            ItemKind::Fern => "minecraft:fern",
            ItemKind::Azalea => "minecraft:azalea",
            ItemKind::FloweringAzalea => "minecraft:flowering_azalea",
            ItemKind::DeadBush => "minecraft:dead_bush",
            ItemKind::Seagrass => "minecraft:seagrass",
            ItemKind::SeaPickle => "minecraft:sea_pickle",
            ItemKind::WhiteWool => "minecraft:white_wool",
            ItemKind::OrangeWool => "minecraft:orange_wool",
            ItemKind::MagentaWool => "minecraft:magenta_wool",
            ItemKind::LightBlueWool => "minecraft:light_blue_wool",
            ItemKind::YellowWool => "minecraft:yellow_wool",
            ItemKind::LimeWool => "minecraft:lime_wool",
            ItemKind::PinkWool => "minecraft:pink_wool",
            ItemKind::GrayWool => "minecraft:gray_wool",
            ItemKind::LightGrayWool => "minecraft:light_gray_wool",
            ItemKind::CyanWool => "minecraft:cyan_wool",
            ItemKind::PurpleWool => "minecraft:purple_wool",
            ItemKind::BlueWool => "minecraft:blue_wool",
            ItemKind::BrownWool => "minecraft:brown_wool",
            ItemKind::GreenWool => "minecraft:green_wool",
            ItemKind::RedWool => "minecraft:red_wool",
            ItemKind::BlackWool => "minecraft:black_wool",
            ItemKind::Dandelion => "minecraft:dandelion",
            ItemKind::Poppy => "minecraft:poppy",
            ItemKind::BlueOrchid => "minecraft:blue_orchid",
            ItemKind::Allium => "minecraft:allium",
            ItemKind::AzureBluet => "minecraft:azure_bluet",
            ItemKind::RedTulip => "minecraft:red_tulip",
            ItemKind::OrangeTulip => "minecraft:orange_tulip",
            ItemKind::WhiteTulip => "minecraft:white_tulip",
            ItemKind::PinkTulip => "minecraft:pink_tulip",
            ItemKind::OxeyeDaisy => "minecraft:oxeye_daisy",
            ItemKind::Cornflower => "minecraft:cornflower",
            ItemKind::LilyOfTheValley => "minecraft:lily_of_the_valley",
            ItemKind::WitherRose => "minecraft:wither_rose",
            ItemKind::SporeBlossom => "minecraft:spore_blossom",
            ItemKind::BrownMushroom => "minecraft:brown_mushroom",
            ItemKind::RedMushroom => "minecraft:red_mushroom",
            ItemKind::CrimsonFungus => "minecraft:crimson_fungus",
            ItemKind::WarpedFungus => "minecraft:warped_fungus",
            ItemKind::CrimsonRoots => "minecraft:crimson_roots",
            ItemKind::WarpedRoots => "minecraft:warped_roots",
            ItemKind::NetherSprouts => "minecraft:nether_sprouts",
            ItemKind::WeepingVines => "minecraft:weeping_vines",
            ItemKind::TwistingVines => "minecraft:twisting_vines",
            ItemKind::SugarCane => "minecraft:sugar_cane",
            ItemKind::Kelp => "minecraft:kelp",
            ItemKind::MossCarpet => "minecraft:moss_carpet",
            ItemKind::MossBlock => "minecraft:moss_block",
            ItemKind::HangingRoots => "minecraft:hanging_roots",
            ItemKind::BigDripleaf => "minecraft:big_dripleaf",
            ItemKind::SmallDripleaf => "minecraft:small_dripleaf",
            ItemKind::Bamboo => "minecraft:bamboo",
            ItemKind::OakSlab => "minecraft:oak_slab",
            ItemKind::SpruceSlab => "minecraft:spruce_slab",
            ItemKind::BirchSlab => "minecraft:birch_slab",
            ItemKind::JungleSlab => "minecraft:jungle_slab",
            ItemKind::AcaciaSlab => "minecraft:acacia_slab",
            ItemKind::DarkOakSlab => "minecraft:dark_oak_slab",
            ItemKind::CrimsonSlab => "minecraft:crimson_slab",
            ItemKind::WarpedSlab => "minecraft:warped_slab",
            ItemKind::StoneSlab => "minecraft:stone_slab",
            ItemKind::SmoothStoneSlab => "minecraft:smooth_stone_slab",
            ItemKind::SandstoneSlab => "minecraft:sandstone_slab",
            ItemKind::CutSandstoneSlab => "minecraft:cut_sandstone_slab",
            ItemKind::PetrifiedOakSlab => "minecraft:petrified_oak_slab",
            ItemKind::CobblestoneSlab => "minecraft:cobblestone_slab",
            ItemKind::BrickSlab => "minecraft:brick_slab",
            ItemKind::StoneBrickSlab => "minecraft:stone_brick_slab",
            ItemKind::NetherBrickSlab => "minecraft:nether_brick_slab",
            ItemKind::QuartzSlab => "minecraft:quartz_slab",
            ItemKind::RedSandstoneSlab => "minecraft:red_sandstone_slab",
            ItemKind::CutRedSandstoneSlab => "minecraft:cut_red_sandstone_slab",
            ItemKind::PurpurSlab => "minecraft:purpur_slab",
            ItemKind::PrismarineSlab => "minecraft:prismarine_slab",
            ItemKind::PrismarineBrickSlab => "minecraft:prismarine_brick_slab",
            ItemKind::DarkPrismarineSlab => "minecraft:dark_prismarine_slab",
            ItemKind::SmoothQuartz => "minecraft:smooth_quartz",
            ItemKind::SmoothRedSandstone => "minecraft:smooth_red_sandstone",
            ItemKind::SmoothSandstone => "minecraft:smooth_sandstone",
            ItemKind::SmoothStone => "minecraft:smooth_stone",
            ItemKind::Bricks => "minecraft:bricks",
            ItemKind::Bookshelf => "minecraft:bookshelf",
            ItemKind::MossyCobblestone => "minecraft:mossy_cobblestone",
            ItemKind::Obsidian => "minecraft:obsidian",
            ItemKind::Torch => "minecraft:torch",
            ItemKind::EndRod => "minecraft:end_rod",
            ItemKind::ChorusPlant => "minecraft:chorus_plant",
            ItemKind::ChorusFlower => "minecraft:chorus_flower",
            ItemKind::PurpurBlock => "minecraft:purpur_block",
            ItemKind::PurpurPillar => "minecraft:purpur_pillar",
            ItemKind::PurpurStairs => "minecraft:purpur_stairs",
            ItemKind::Spawner => "minecraft:spawner",
            ItemKind::OakStairs => "minecraft:oak_stairs",
            ItemKind::Chest => "minecraft:chest",
            ItemKind::CraftingTable => "minecraft:crafting_table",
            ItemKind::Farmland => "minecraft:farmland",
            ItemKind::Furnace => "minecraft:furnace",
            ItemKind::Ladder => "minecraft:ladder",
            ItemKind::CobblestoneStairs => "minecraft:cobblestone_stairs",
            ItemKind::Snow => "minecraft:snow",
            ItemKind::Ice => "minecraft:ice",
            ItemKind::SnowBlock => "minecraft:snow_block",
            ItemKind::Cactus => "minecraft:cactus",
            ItemKind::Clay => "minecraft:clay",
            ItemKind::Jukebox => "minecraft:jukebox",
            ItemKind::OakFence => "minecraft:oak_fence",
            ItemKind::SpruceFence => "minecraft:spruce_fence",
            ItemKind::BirchFence => "minecraft:birch_fence",
            ItemKind::JungleFence => "minecraft:jungle_fence",
            ItemKind::AcaciaFence => "minecraft:acacia_fence",
            ItemKind::DarkOakFence => "minecraft:dark_oak_fence",
            ItemKind::CrimsonFence => "minecraft:crimson_fence",
            ItemKind::WarpedFence => "minecraft:warped_fence",
            ItemKind::Pumpkin => "minecraft:pumpkin",
            ItemKind::CarvedPumpkin => "minecraft:carved_pumpkin",
            ItemKind::JackOLantern => "minecraft:jack_o_lantern",
            ItemKind::Netherrack => "minecraft:netherrack",
            ItemKind::SoulSand => "minecraft:soul_sand",
            ItemKind::SoulSoil => "minecraft:soul_soil",
            ItemKind::Basalt => "minecraft:basalt",
            ItemKind::PolishedBasalt => "minecraft:polished_basalt",
            ItemKind::SmoothBasalt => "minecraft:smooth_basalt",
            ItemKind::SoulTorch => "minecraft:soul_torch",
            ItemKind::Glowstone => "minecraft:glowstone",
            ItemKind::InfestedStone => "minecraft:infested_stone",
            ItemKind::InfestedCobblestone => "minecraft:infested_cobblestone",
            ItemKind::InfestedStoneBricks => "minecraft:infested_stone_bricks",
            ItemKind::InfestedMossyStoneBricks => "minecraft:infested_mossy_stone_bricks",
            ItemKind::InfestedCrackedStoneBricks => "minecraft:infested_cracked_stone_bricks",
            ItemKind::InfestedChiseledStoneBricks => "minecraft:infested_chiseled_stone_bricks",
            ItemKind::InfestedDeepslate => "minecraft:infested_deepslate",
            ItemKind::StoneBricks => "minecraft:stone_bricks",
            ItemKind::MossyStoneBricks => "minecraft:mossy_stone_bricks",
            ItemKind::CrackedStoneBricks => "minecraft:cracked_stone_bricks",
            ItemKind::ChiseledStoneBricks => "minecraft:chiseled_stone_bricks",
            ItemKind::DeepslateBricks => "minecraft:deepslate_bricks",
            ItemKind::CrackedDeepslateBricks => "minecraft:cracked_deepslate_bricks",
            ItemKind::DeepslateTiles => "minecraft:deepslate_tiles",
            ItemKind::CrackedDeepslateTiles => "minecraft:cracked_deepslate_tiles",
            ItemKind::ChiseledDeepslate => "minecraft:chiseled_deepslate",
            ItemKind::BrownMushroomBlock => "minecraft:brown_mushroom_block",
            ItemKind::RedMushroomBlock => "minecraft:red_mushroom_block",
            ItemKind::MushroomStem => "minecraft:mushroom_stem",
            ItemKind::IronBars => "minecraft:iron_bars",
            ItemKind::Chain => "minecraft:chain",
            ItemKind::GlassPane => "minecraft:glass_pane",
            ItemKind::Melon => "minecraft:melon",
            ItemKind::Vine => "minecraft:vine",
            ItemKind::GlowLichen => "minecraft:glow_lichen",
            ItemKind::BrickStairs => "minecraft:brick_stairs",
            ItemKind::StoneBrickStairs => "minecraft:stone_brick_stairs",
            ItemKind::Mycelium => "minecraft:mycelium",
            ItemKind::LilyPad => "minecraft:lily_pad",
            ItemKind::NetherBricks => "minecraft:nether_bricks",
            ItemKind::CrackedNetherBricks => "minecraft:cracked_nether_bricks",
            ItemKind::ChiseledNetherBricks => "minecraft:chiseled_nether_bricks",
            ItemKind::NetherBrickFence => "minecraft:nether_brick_fence",
            ItemKind::NetherBrickStairs => "minecraft:nether_brick_stairs",
            ItemKind::EnchantingTable => "minecraft:enchanting_table",
            ItemKind::EndPortalFrame => "minecraft:end_portal_frame",
            ItemKind::EndStone => "minecraft:end_stone",
            ItemKind::EndStoneBricks => "minecraft:end_stone_bricks",
            ItemKind::DragonEgg => "minecraft:dragon_egg",
            ItemKind::SandstoneStairs => "minecraft:sandstone_stairs",
            ItemKind::EnderChest => "minecraft:ender_chest",
            ItemKind::EmeraldBlock => "minecraft:emerald_block",
            ItemKind::SpruceStairs => "minecraft:spruce_stairs",
            ItemKind::BirchStairs => "minecraft:birch_stairs",
            ItemKind::JungleStairs => "minecraft:jungle_stairs",
            ItemKind::CrimsonStairs => "minecraft:crimson_stairs",
            ItemKind::WarpedStairs => "minecraft:warped_stairs",
            ItemKind::CommandBlock => "minecraft:command_block",
            ItemKind::Beacon => "minecraft:beacon",
            ItemKind::CobblestoneWall => "minecraft:cobblestone_wall",
            ItemKind::MossyCobblestoneWall => "minecraft:mossy_cobblestone_wall",
            ItemKind::BrickWall => "minecraft:brick_wall",
            ItemKind::PrismarineWall => "minecraft:prismarine_wall",
            ItemKind::RedSandstoneWall => "minecraft:red_sandstone_wall",
            ItemKind::MossyStoneBrickWall => "minecraft:mossy_stone_brick_wall",
            ItemKind::GraniteWall => "minecraft:granite_wall",
            ItemKind::StoneBrickWall => "minecraft:stone_brick_wall",
            ItemKind::NetherBrickWall => "minecraft:nether_brick_wall",
            ItemKind::AndesiteWall => "minecraft:andesite_wall",
            ItemKind::RedNetherBrickWall => "minecraft:red_nether_brick_wall",
            ItemKind::SandstoneWall => "minecraft:sandstone_wall",
            ItemKind::EndStoneBrickWall => "minecraft:end_stone_brick_wall",
            ItemKind::DioriteWall => "minecraft:diorite_wall",
            ItemKind::BlackstoneWall => "minecraft:blackstone_wall",
            ItemKind::PolishedBlackstoneWall => "minecraft:polished_blackstone_wall",
            ItemKind::PolishedBlackstoneBrickWall => "minecraft:polished_blackstone_brick_wall",
            ItemKind::CobbledDeepslateWall => "minecraft:cobbled_deepslate_wall",
            ItemKind::PolishedDeepslateWall => "minecraft:polished_deepslate_wall",
            ItemKind::DeepslateBrickWall => "minecraft:deepslate_brick_wall",
            ItemKind::DeepslateTileWall => "minecraft:deepslate_tile_wall",
            ItemKind::Anvil => "minecraft:anvil",
            ItemKind::ChippedAnvil => "minecraft:chipped_anvil",
            ItemKind::DamagedAnvil => "minecraft:damaged_anvil",
            ItemKind::ChiseledQuartzBlock => "minecraft:chiseled_quartz_block",
            ItemKind::QuartzBlock => "minecraft:quartz_block",
            ItemKind::QuartzBricks => "minecraft:quartz_bricks",
            ItemKind::QuartzPillar => "minecraft:quartz_pillar",
            ItemKind::QuartzStairs => "minecraft:quartz_stairs",
            ItemKind::WhiteTerracotta => "minecraft:white_terracotta",
            ItemKind::OrangeTerracotta => "minecraft:orange_terracotta",
            ItemKind::MagentaTerracotta => "minecraft:magenta_terracotta",
            ItemKind::LightBlueTerracotta => "minecraft:light_blue_terracotta",
            ItemKind::YellowTerracotta => "minecraft:yellow_terracotta",
            ItemKind::LimeTerracotta => "minecraft:lime_terracotta",
            ItemKind::PinkTerracotta => "minecraft:pink_terracotta",
            ItemKind::GrayTerracotta => "minecraft:gray_terracotta",
            ItemKind::LightGrayTerracotta => "minecraft:light_gray_terracotta",
            ItemKind::CyanTerracotta => "minecraft:cyan_terracotta",
            ItemKind::PurpleTerracotta => "minecraft:purple_terracotta",
            ItemKind::BlueTerracotta => "minecraft:blue_terracotta",
            ItemKind::BrownTerracotta => "minecraft:brown_terracotta",
            ItemKind::GreenTerracotta => "minecraft:green_terracotta",
            ItemKind::RedTerracotta => "minecraft:red_terracotta",
            ItemKind::BlackTerracotta => "minecraft:black_terracotta",
            ItemKind::Barrier => "minecraft:barrier",
            ItemKind::Light => "minecraft:light",
            ItemKind::HayBlock => "minecraft:hay_block",
            ItemKind::WhiteCarpet => "minecraft:white_carpet",
            ItemKind::OrangeCarpet => "minecraft:orange_carpet",
            ItemKind::MagentaCarpet => "minecraft:magenta_carpet",
            ItemKind::LightBlueCarpet => "minecraft:light_blue_carpet",
            ItemKind::YellowCarpet => "minecraft:yellow_carpet",
            ItemKind::LimeCarpet => "minecraft:lime_carpet",
            ItemKind::PinkCarpet => "minecraft:pink_carpet",
            ItemKind::GrayCarpet => "minecraft:gray_carpet",
            ItemKind::LightGrayCarpet => "minecraft:light_gray_carpet",
            ItemKind::CyanCarpet => "minecraft:cyan_carpet",
            ItemKind::PurpleCarpet => "minecraft:purple_carpet",
            ItemKind::BlueCarpet => "minecraft:blue_carpet",
            ItemKind::BrownCarpet => "minecraft:brown_carpet",
            ItemKind::GreenCarpet => "minecraft:green_carpet",
            ItemKind::RedCarpet => "minecraft:red_carpet",
            ItemKind::BlackCarpet => "minecraft:black_carpet",
            ItemKind::Terracotta => "minecraft:terracotta",
            ItemKind::PackedIce => "minecraft:packed_ice",
            ItemKind::AcaciaStairs => "minecraft:acacia_stairs",
            ItemKind::DarkOakStairs => "minecraft:dark_oak_stairs",
            ItemKind::DirtPath => "minecraft:dirt_path",
            ItemKind::Sunflower => "minecraft:sunflower",
            ItemKind::Lilac => "minecraft:lilac",
            ItemKind::RoseBush => "minecraft:rose_bush",
            ItemKind::Peony => "minecraft:peony",
            ItemKind::TallGrass => "minecraft:tall_grass",
            ItemKind::LargeFern => "minecraft:large_fern",
            ItemKind::WhiteStainedGlass => "minecraft:white_stained_glass",
            ItemKind::OrangeStainedGlass => "minecraft:orange_stained_glass",
            ItemKind::MagentaStainedGlass => "minecraft:magenta_stained_glass",
            ItemKind::LightBlueStainedGlass => "minecraft:light_blue_stained_glass",
            ItemKind::YellowStainedGlass => "minecraft:yellow_stained_glass",
            ItemKind::LimeStainedGlass => "minecraft:lime_stained_glass",
            ItemKind::PinkStainedGlass => "minecraft:pink_stained_glass",
            ItemKind::GrayStainedGlass => "minecraft:gray_stained_glass",
            ItemKind::LightGrayStainedGlass => "minecraft:light_gray_stained_glass",
            ItemKind::CyanStainedGlass => "minecraft:cyan_stained_glass",
            ItemKind::PurpleStainedGlass => "minecraft:purple_stained_glass",
            ItemKind::BlueStainedGlass => "minecraft:blue_stained_glass",
            ItemKind::BrownStainedGlass => "minecraft:brown_stained_glass",
            ItemKind::GreenStainedGlass => "minecraft:green_stained_glass",
            ItemKind::RedStainedGlass => "minecraft:red_stained_glass",
            ItemKind::BlackStainedGlass => "minecraft:black_stained_glass",
            ItemKind::WhiteStainedGlassPane => "minecraft:white_stained_glass_pane",
            ItemKind::OrangeStainedGlassPane => "minecraft:orange_stained_glass_pane",
            ItemKind::MagentaStainedGlassPane => "minecraft:magenta_stained_glass_pane",
            ItemKind::LightBlueStainedGlassPane => "minecraft:light_blue_stained_glass_pane",
            ItemKind::YellowStainedGlassPane => "minecraft:yellow_stained_glass_pane",
            ItemKind::LimeStainedGlassPane => "minecraft:lime_stained_glass_pane",
            ItemKind::PinkStainedGlassPane => "minecraft:pink_stained_glass_pane",
            ItemKind::GrayStainedGlassPane => "minecraft:gray_stained_glass_pane",
            ItemKind::LightGrayStainedGlassPane => "minecraft:light_gray_stained_glass_pane",
            ItemKind::CyanStainedGlassPane => "minecraft:cyan_stained_glass_pane",
            ItemKind::PurpleStainedGlassPane => "minecraft:purple_stained_glass_pane",
            ItemKind::BlueStainedGlassPane => "minecraft:blue_stained_glass_pane",
            ItemKind::BrownStainedGlassPane => "minecraft:brown_stained_glass_pane",
            ItemKind::GreenStainedGlassPane => "minecraft:green_stained_glass_pane",
            ItemKind::RedStainedGlassPane => "minecraft:red_stained_glass_pane",
            ItemKind::BlackStainedGlassPane => "minecraft:black_stained_glass_pane",
            ItemKind::Prismarine => "minecraft:prismarine",
            ItemKind::PrismarineBricks => "minecraft:prismarine_bricks",
            ItemKind::DarkPrismarine => "minecraft:dark_prismarine",
            ItemKind::PrismarineStairs => "minecraft:prismarine_stairs",
            ItemKind::PrismarineBrickStairs => "minecraft:prismarine_brick_stairs",
            ItemKind::DarkPrismarineStairs => "minecraft:dark_prismarine_stairs",
            ItemKind::SeaLantern => "minecraft:sea_lantern",
            ItemKind::RedSandstone => "minecraft:red_sandstone",
            ItemKind::ChiseledRedSandstone => "minecraft:chiseled_red_sandstone",
            ItemKind::CutRedSandstone => "minecraft:cut_red_sandstone",
            ItemKind::RedSandstoneStairs => "minecraft:red_sandstone_stairs",
            ItemKind::RepeatingCommandBlock => "minecraft:repeating_command_block",
            ItemKind::ChainCommandBlock => "minecraft:chain_command_block",
            ItemKind::MagmaBlock => "minecraft:magma_block",
            ItemKind::NetherWartBlock => "minecraft:nether_wart_block",
            ItemKind::WarpedWartBlock => "minecraft:warped_wart_block",
            ItemKind::RedNetherBricks => "minecraft:red_nether_bricks",
            ItemKind::BoneBlock => "minecraft:bone_block",
            ItemKind::StructureVoid => "minecraft:structure_void",
            ItemKind::ShulkerBox => "minecraft:shulker_box",
            ItemKind::WhiteShulkerBox => "minecraft:white_shulker_box",
            ItemKind::OrangeShulkerBox => "minecraft:orange_shulker_box",
            ItemKind::MagentaShulkerBox => "minecraft:magenta_shulker_box",
            ItemKind::LightBlueShulkerBox => "minecraft:light_blue_shulker_box",
            ItemKind::YellowShulkerBox => "minecraft:yellow_shulker_box",
            ItemKind::LimeShulkerBox => "minecraft:lime_shulker_box",
            ItemKind::PinkShulkerBox => "minecraft:pink_shulker_box",
            ItemKind::GrayShulkerBox => "minecraft:gray_shulker_box",
            ItemKind::LightGrayShulkerBox => "minecraft:light_gray_shulker_box",
            ItemKind::CyanShulkerBox => "minecraft:cyan_shulker_box",
            ItemKind::PurpleShulkerBox => "minecraft:purple_shulker_box",
            ItemKind::BlueShulkerBox => "minecraft:blue_shulker_box",
            ItemKind::BrownShulkerBox => "minecraft:brown_shulker_box",
            ItemKind::GreenShulkerBox => "minecraft:green_shulker_box",
            ItemKind::RedShulkerBox => "minecraft:red_shulker_box",
            ItemKind::BlackShulkerBox => "minecraft:black_shulker_box",
            ItemKind::WhiteGlazedTerracotta => "minecraft:white_glazed_terracotta",
            ItemKind::OrangeGlazedTerracotta => "minecraft:orange_glazed_terracotta",
            ItemKind::MagentaGlazedTerracotta => "minecraft:magenta_glazed_terracotta",
            ItemKind::LightBlueGlazedTerracotta => "minecraft:light_blue_glazed_terracotta",
            ItemKind::YellowGlazedTerracotta => "minecraft:yellow_glazed_terracotta",
            ItemKind::LimeGlazedTerracotta => "minecraft:lime_glazed_terracotta",
            ItemKind::PinkGlazedTerracotta => "minecraft:pink_glazed_terracotta",
            ItemKind::GrayGlazedTerracotta => "minecraft:gray_glazed_terracotta",
            ItemKind::LightGrayGlazedTerracotta => "minecraft:light_gray_glazed_terracotta",
            ItemKind::CyanGlazedTerracotta => "minecraft:cyan_glazed_terracotta",
            ItemKind::PurpleGlazedTerracotta => "minecraft:purple_glazed_terracotta",
            ItemKind::BlueGlazedTerracotta => "minecraft:blue_glazed_terracotta",
            ItemKind::BrownGlazedTerracotta => "minecraft:brown_glazed_terracotta",
            ItemKind::GreenGlazedTerracotta => "minecraft:green_glazed_terracotta",
            ItemKind::RedGlazedTerracotta => "minecraft:red_glazed_terracotta",
            ItemKind::BlackGlazedTerracotta => "minecraft:black_glazed_terracotta",
            ItemKind::WhiteConcrete => "minecraft:white_concrete",
            ItemKind::OrangeConcrete => "minecraft:orange_concrete",
            ItemKind::MagentaConcrete => "minecraft:magenta_concrete",
            ItemKind::LightBlueConcrete => "minecraft:light_blue_concrete",
            ItemKind::YellowConcrete => "minecraft:yellow_concrete",
            ItemKind::LimeConcrete => "minecraft:lime_concrete",
            ItemKind::PinkConcrete => "minecraft:pink_concrete",
            ItemKind::GrayConcrete => "minecraft:gray_concrete",
            ItemKind::LightGrayConcrete => "minecraft:light_gray_concrete",
            ItemKind::CyanConcrete => "minecraft:cyan_concrete",
            ItemKind::PurpleConcrete => "minecraft:purple_concrete",
            ItemKind::BlueConcrete => "minecraft:blue_concrete",
            ItemKind::BrownConcrete => "minecraft:brown_concrete",
            ItemKind::GreenConcrete => "minecraft:green_concrete",
            ItemKind::RedConcrete => "minecraft:red_concrete",
            ItemKind::BlackConcrete => "minecraft:black_concrete",
            ItemKind::WhiteConcretePowder => "minecraft:white_concrete_powder",
            ItemKind::OrangeConcretePowder => "minecraft:orange_concrete_powder",
            ItemKind::MagentaConcretePowder => "minecraft:magenta_concrete_powder",
            ItemKind::LightBlueConcretePowder => "minecraft:light_blue_concrete_powder",
            ItemKind::YellowConcretePowder => "minecraft:yellow_concrete_powder",
            ItemKind::LimeConcretePowder => "minecraft:lime_concrete_powder",
            ItemKind::PinkConcretePowder => "minecraft:pink_concrete_powder",
            ItemKind::GrayConcretePowder => "minecraft:gray_concrete_powder",
            ItemKind::LightGrayConcretePowder => "minecraft:light_gray_concrete_powder",
            ItemKind::CyanConcretePowder => "minecraft:cyan_concrete_powder",
            ItemKind::PurpleConcretePowder => "minecraft:purple_concrete_powder",
            ItemKind::BlueConcretePowder => "minecraft:blue_concrete_powder",
            ItemKind::BrownConcretePowder => "minecraft:brown_concrete_powder",
            ItemKind::GreenConcretePowder => "minecraft:green_concrete_powder",
            ItemKind::RedConcretePowder => "minecraft:red_concrete_powder",
            ItemKind::BlackConcretePowder => "minecraft:black_concrete_powder",
            ItemKind::TurtleEgg => "minecraft:turtle_egg",
            ItemKind::DeadTubeCoralBlock => "minecraft:dead_tube_coral_block",
            ItemKind::DeadBrainCoralBlock => "minecraft:dead_brain_coral_block",
            ItemKind::DeadBubbleCoralBlock => "minecraft:dead_bubble_coral_block",
            ItemKind::DeadFireCoralBlock => "minecraft:dead_fire_coral_block",
            ItemKind::DeadHornCoralBlock => "minecraft:dead_horn_coral_block",
            ItemKind::TubeCoralBlock => "minecraft:tube_coral_block",
            ItemKind::BrainCoralBlock => "minecraft:brain_coral_block",
            ItemKind::BubbleCoralBlock => "minecraft:bubble_coral_block",
            ItemKind::FireCoralBlock => "minecraft:fire_coral_block",
            ItemKind::HornCoralBlock => "minecraft:horn_coral_block",
            ItemKind::TubeCoral => "minecraft:tube_coral",
            ItemKind::BrainCoral => "minecraft:brain_coral",
            ItemKind::BubbleCoral => "minecraft:bubble_coral",
            ItemKind::FireCoral => "minecraft:fire_coral",
            ItemKind::HornCoral => "minecraft:horn_coral",
            ItemKind::DeadBrainCoral => "minecraft:dead_brain_coral",
            ItemKind::DeadBubbleCoral => "minecraft:dead_bubble_coral",
            ItemKind::DeadFireCoral => "minecraft:dead_fire_coral",
            ItemKind::DeadHornCoral => "minecraft:dead_horn_coral",
            ItemKind::DeadTubeCoral => "minecraft:dead_tube_coral",
            ItemKind::TubeCoralFan => "minecraft:tube_coral_fan",
            ItemKind::BrainCoralFan => "minecraft:brain_coral_fan",
            ItemKind::BubbleCoralFan => "minecraft:bubble_coral_fan",
            ItemKind::FireCoralFan => "minecraft:fire_coral_fan",
            ItemKind::HornCoralFan => "minecraft:horn_coral_fan",
            ItemKind::DeadTubeCoralFan => "minecraft:dead_tube_coral_fan",
            ItemKind::DeadBrainCoralFan => "minecraft:dead_brain_coral_fan",
            ItemKind::DeadBubbleCoralFan => "minecraft:dead_bubble_coral_fan",
            ItemKind::DeadFireCoralFan => "minecraft:dead_fire_coral_fan",
            ItemKind::DeadHornCoralFan => "minecraft:dead_horn_coral_fan",
            ItemKind::BlueIce => "minecraft:blue_ice",
            ItemKind::Conduit => "minecraft:conduit",
            ItemKind::PolishedGraniteStairs => "minecraft:polished_granite_stairs",
            ItemKind::SmoothRedSandstoneStairs => "minecraft:smooth_red_sandstone_stairs",
            ItemKind::MossyStoneBrickStairs => "minecraft:mossy_stone_brick_stairs",
            ItemKind::PolishedDioriteStairs => "minecraft:polished_diorite_stairs",
            ItemKind::MossyCobblestoneStairs => "minecraft:mossy_cobblestone_stairs",
            ItemKind::EndStoneBrickStairs => "minecraft:end_stone_brick_stairs",
            ItemKind::StoneStairs => "minecraft:stone_stairs",
            ItemKind::SmoothSandstoneStairs => "minecraft:smooth_sandstone_stairs",
            ItemKind::SmoothQuartzStairs => "minecraft:smooth_quartz_stairs",
            ItemKind::GraniteStairs => "minecraft:granite_stairs",
            ItemKind::AndesiteStairs => "minecraft:andesite_stairs",
            ItemKind::RedNetherBrickStairs => "minecraft:red_nether_brick_stairs",
            ItemKind::PolishedAndesiteStairs => "minecraft:polished_andesite_stairs",
            ItemKind::DioriteStairs => "minecraft:diorite_stairs",
            ItemKind::CobbledDeepslateStairs => "minecraft:cobbled_deepslate_stairs",
            ItemKind::PolishedDeepslateStairs => "minecraft:polished_deepslate_stairs",
            ItemKind::DeepslateBrickStairs => "minecraft:deepslate_brick_stairs",
            ItemKind::DeepslateTileStairs => "minecraft:deepslate_tile_stairs",
            ItemKind::PolishedGraniteSlab => "minecraft:polished_granite_slab",
            ItemKind::SmoothRedSandstoneSlab => "minecraft:smooth_red_sandstone_slab",
            ItemKind::MossyStoneBrickSlab => "minecraft:mossy_stone_brick_slab",
            ItemKind::PolishedDioriteSlab => "minecraft:polished_diorite_slab",
            ItemKind::MossyCobblestoneSlab => "minecraft:mossy_cobblestone_slab",
            ItemKind::EndStoneBrickSlab => "minecraft:end_stone_brick_slab",
            ItemKind::SmoothSandstoneSlab => "minecraft:smooth_sandstone_slab",
            ItemKind::SmoothQuartzSlab => "minecraft:smooth_quartz_slab",
            ItemKind::GraniteSlab => "minecraft:granite_slab",
            ItemKind::AndesiteSlab => "minecraft:andesite_slab",
            ItemKind::RedNetherBrickSlab => "minecraft:red_nether_brick_slab",
            ItemKind::PolishedAndesiteSlab => "minecraft:polished_andesite_slab",
            ItemKind::DioriteSlab => "minecraft:diorite_slab",
            ItemKind::CobbledDeepslateSlab => "minecraft:cobbled_deepslate_slab",
            ItemKind::PolishedDeepslateSlab => "minecraft:polished_deepslate_slab",
            ItemKind::DeepslateBrickSlab => "minecraft:deepslate_brick_slab",
            ItemKind::DeepslateTileSlab => "minecraft:deepslate_tile_slab",
            ItemKind::Scaffolding => "minecraft:scaffolding",
            ItemKind::Redstone => "minecraft:redstone",
            ItemKind::RedstoneTorch => "minecraft:redstone_torch",
            ItemKind::RedstoneBlock => "minecraft:redstone_block",
            ItemKind::Repeater => "minecraft:repeater",
            ItemKind::Comparator => "minecraft:comparator",
            ItemKind::Piston => "minecraft:piston",
            ItemKind::StickyPiston => "minecraft:sticky_piston",
            ItemKind::SlimeBlock => "minecraft:slime_block",
            ItemKind::HoneyBlock => "minecraft:honey_block",
            ItemKind::Observer => "minecraft:observer",
            ItemKind::Hopper => "minecraft:hopper",
            ItemKind::Dispenser => "minecraft:dispenser",
            ItemKind::Dropper => "minecraft:dropper",
            ItemKind::Lectern => "minecraft:lectern",
            ItemKind::Target => "minecraft:target",
            ItemKind::Lever => "minecraft:lever",
            ItemKind::LightningRod => "minecraft:lightning_rod",
            ItemKind::DaylightDetector => "minecraft:daylight_detector",
            ItemKind::SculkSensor => "minecraft:sculk_sensor",
            ItemKind::TripwireHook => "minecraft:tripwire_hook",
            ItemKind::TrappedChest => "minecraft:trapped_chest",
            ItemKind::Tnt => "minecraft:tnt",
            ItemKind::RedstoneLamp => "minecraft:redstone_lamp",
            ItemKind::NoteBlock => "minecraft:note_block",
            ItemKind::StoneButton => "minecraft:stone_button",
            ItemKind::PolishedBlackstoneButton => "minecraft:polished_blackstone_button",
            ItemKind::OakButton => "minecraft:oak_button",
            ItemKind::SpruceButton => "minecraft:spruce_button",
            ItemKind::BirchButton => "minecraft:birch_button",
            ItemKind::JungleButton => "minecraft:jungle_button",
            ItemKind::AcaciaButton => "minecraft:acacia_button",
            ItemKind::DarkOakButton => "minecraft:dark_oak_button",
            ItemKind::CrimsonButton => "minecraft:crimson_button",
            ItemKind::WarpedButton => "minecraft:warped_button",
            ItemKind::StonePressurePlate => "minecraft:stone_pressure_plate",
            ItemKind::PolishedBlackstonePressurePlate => {
                "minecraft:polished_blackstone_pressure_plate"
            }
            ItemKind::LightWeightedPressurePlate => "minecraft:light_weighted_pressure_plate",
            ItemKind::HeavyWeightedPressurePlate => "minecraft:heavy_weighted_pressure_plate",
            ItemKind::OakPressurePlate => "minecraft:oak_pressure_plate",
            ItemKind::SprucePressurePlate => "minecraft:spruce_pressure_plate",
            ItemKind::BirchPressurePlate => "minecraft:birch_pressure_plate",
            ItemKind::JunglePressurePlate => "minecraft:jungle_pressure_plate",
            ItemKind::AcaciaPressurePlate => "minecraft:acacia_pressure_plate",
            ItemKind::DarkOakPressurePlate => "minecraft:dark_oak_pressure_plate",
            ItemKind::CrimsonPressurePlate => "minecraft:crimson_pressure_plate",
            ItemKind::WarpedPressurePlate => "minecraft:warped_pressure_plate",
            ItemKind::IronDoor => "minecraft:iron_door",
            ItemKind::OakDoor => "minecraft:oak_door",
            ItemKind::SpruceDoor => "minecraft:spruce_door",
            ItemKind::BirchDoor => "minecraft:birch_door",
            ItemKind::JungleDoor => "minecraft:jungle_door",
            ItemKind::AcaciaDoor => "minecraft:acacia_door",
            ItemKind::DarkOakDoor => "minecraft:dark_oak_door",
            ItemKind::CrimsonDoor => "minecraft:crimson_door",
            ItemKind::WarpedDoor => "minecraft:warped_door",
            ItemKind::IronTrapdoor => "minecraft:iron_trapdoor",
            ItemKind::OakTrapdoor => "minecraft:oak_trapdoor",
            ItemKind::SpruceTrapdoor => "minecraft:spruce_trapdoor",
            ItemKind::BirchTrapdoor => "minecraft:birch_trapdoor",
            ItemKind::JungleTrapdoor => "minecraft:jungle_trapdoor",
            ItemKind::AcaciaTrapdoor => "minecraft:acacia_trapdoor",
            ItemKind::DarkOakTrapdoor => "minecraft:dark_oak_trapdoor",
            ItemKind::CrimsonTrapdoor => "minecraft:crimson_trapdoor",
            ItemKind::WarpedTrapdoor => "minecraft:warped_trapdoor",
            ItemKind::OakFenceGate => "minecraft:oak_fence_gate",
            ItemKind::SpruceFenceGate => "minecraft:spruce_fence_gate",
            ItemKind::BirchFenceGate => "minecraft:birch_fence_gate",
            ItemKind::JungleFenceGate => "minecraft:jungle_fence_gate",
            ItemKind::AcaciaFenceGate => "minecraft:acacia_fence_gate",
            ItemKind::DarkOakFenceGate => "minecraft:dark_oak_fence_gate",
            ItemKind::CrimsonFenceGate => "minecraft:crimson_fence_gate",
            ItemKind::WarpedFenceGate => "minecraft:warped_fence_gate",
            ItemKind::PoweredRail => "minecraft:powered_rail",
            ItemKind::DetectorRail => "minecraft:detector_rail",
            ItemKind::Rail => "minecraft:rail",
            ItemKind::ActivatorRail => "minecraft:activator_rail",
            ItemKind::Saddle => "minecraft:saddle",
            ItemKind::Minecart => "minecraft:minecart",
            ItemKind::ChestMinecart => "minecraft:chest_minecart",
            ItemKind::FurnaceMinecart => "minecraft:furnace_minecart",
            ItemKind::TntMinecart => "minecraft:tnt_minecart",
            ItemKind::HopperMinecart => "minecraft:hopper_minecart",
            ItemKind::CarrotOnAStick => "minecraft:carrot_on_a_stick",
            ItemKind::WarpedFungusOnAStick => "minecraft:warped_fungus_on_a_stick",
            ItemKind::Elytra => "minecraft:elytra",
            ItemKind::OakBoat => "minecraft:oak_boat",
            ItemKind::SpruceBoat => "minecraft:spruce_boat",
            ItemKind::BirchBoat => "minecraft:birch_boat",
            ItemKind::JungleBoat => "minecraft:jungle_boat",
            ItemKind::AcaciaBoat => "minecraft:acacia_boat",
            ItemKind::DarkOakBoat => "minecraft:dark_oak_boat",
            ItemKind::StructureBlock => "minecraft:structure_block",
            ItemKind::Jigsaw => "minecraft:jigsaw",
            ItemKind::TurtleHelmet => "minecraft:turtle_helmet",
            ItemKind::Scute => "minecraft:scute",
            ItemKind::FlintAndSteel => "minecraft:flint_and_steel",
            ItemKind::Apple => "minecraft:apple",
            ItemKind::Bow => "minecraft:bow",
            ItemKind::Arrow => "minecraft:arrow",
            ItemKind::Coal => "minecraft:coal",
            ItemKind::Charcoal => "minecraft:charcoal",
            ItemKind::Diamond => "minecraft:diamond",
            ItemKind::Emerald => "minecraft:emerald",
            ItemKind::LapisLazuli => "minecraft:lapis_lazuli",
            ItemKind::Quartz => "minecraft:quartz",
            ItemKind::AmethystShard => "minecraft:amethyst_shard",
            ItemKind::RawIron => "minecraft:raw_iron",
            ItemKind::IronIngot => "minecraft:iron_ingot",
            ItemKind::RawCopper => "minecraft:raw_copper",
            ItemKind::CopperIngot => "minecraft:copper_ingot",
            ItemKind::RawGold => "minecraft:raw_gold",
            ItemKind::GoldIngot => "minecraft:gold_ingot",
            ItemKind::NetheriteIngot => "minecraft:netherite_ingot",
            ItemKind::NetheriteScrap => "minecraft:netherite_scrap",
            ItemKind::WoodenSword => "minecraft:wooden_sword",
            ItemKind::WoodenShovel => "minecraft:wooden_shovel",
            ItemKind::WoodenPickaxe => "minecraft:wooden_pickaxe",
            ItemKind::WoodenAxe => "minecraft:wooden_axe",
            ItemKind::WoodenHoe => "minecraft:wooden_hoe",
            ItemKind::StoneSword => "minecraft:stone_sword",
            ItemKind::StoneShovel => "minecraft:stone_shovel",
            ItemKind::StonePickaxe => "minecraft:stone_pickaxe",
            ItemKind::StoneAxe => "minecraft:stone_axe",
            ItemKind::StoneHoe => "minecraft:stone_hoe",
            ItemKind::GoldenSword => "minecraft:golden_sword",
            ItemKind::GoldenShovel => "minecraft:golden_shovel",
            ItemKind::GoldenPickaxe => "minecraft:golden_pickaxe",
            ItemKind::GoldenAxe => "minecraft:golden_axe",
            ItemKind::GoldenHoe => "minecraft:golden_hoe",
            ItemKind::IronSword => "minecraft:iron_sword",
            ItemKind::IronShovel => "minecraft:iron_shovel",
            ItemKind::IronPickaxe => "minecraft:iron_pickaxe",
            ItemKind::IronAxe => "minecraft:iron_axe",
            ItemKind::IronHoe => "minecraft:iron_hoe",
            ItemKind::DiamondSword => "minecraft:diamond_sword",
            ItemKind::DiamondShovel => "minecraft:diamond_shovel",
            ItemKind::DiamondPickaxe => "minecraft:diamond_pickaxe",
            ItemKind::DiamondAxe => "minecraft:diamond_axe",
            ItemKind::DiamondHoe => "minecraft:diamond_hoe",
            ItemKind::NetheriteSword => "minecraft:netherite_sword",
            ItemKind::NetheriteShovel => "minecraft:netherite_shovel",
            ItemKind::NetheritePickaxe => "minecraft:netherite_pickaxe",
            ItemKind::NetheriteAxe => "minecraft:netherite_axe",
            ItemKind::NetheriteHoe => "minecraft:netherite_hoe",
            ItemKind::Stick => "minecraft:stick",
            ItemKind::Bowl => "minecraft:bowl",
            ItemKind::MushroomStew => "minecraft:mushroom_stew",
            ItemKind::String => "minecraft:string",
            ItemKind::Feather => "minecraft:feather",
            ItemKind::Gunpowder => "minecraft:gunpowder",
            ItemKind::WheatSeeds => "minecraft:wheat_seeds",
            ItemKind::Wheat => "minecraft:wheat",
            ItemKind::Bread => "minecraft:bread",
            ItemKind::LeatherHelmet => "minecraft:leather_helmet",
            ItemKind::LeatherChestplate => "minecraft:leather_chestplate",
            ItemKind::LeatherLeggings => "minecraft:leather_leggings",
            ItemKind::LeatherBoots => "minecraft:leather_boots",
            ItemKind::ChainmailHelmet => "minecraft:chainmail_helmet",
            ItemKind::ChainmailChestplate => "minecraft:chainmail_chestplate",
            ItemKind::ChainmailLeggings => "minecraft:chainmail_leggings",
            ItemKind::ChainmailBoots => "minecraft:chainmail_boots",
            ItemKind::IronHelmet => "minecraft:iron_helmet",
            ItemKind::IronChestplate => "minecraft:iron_chestplate",
            ItemKind::IronLeggings => "minecraft:iron_leggings",
            ItemKind::IronBoots => "minecraft:iron_boots",
            ItemKind::DiamondHelmet => "minecraft:diamond_helmet",
            ItemKind::DiamondChestplate => "minecraft:diamond_chestplate",
            ItemKind::DiamondLeggings => "minecraft:diamond_leggings",
            ItemKind::DiamondBoots => "minecraft:diamond_boots",
            ItemKind::GoldenHelmet => "minecraft:golden_helmet",
            ItemKind::GoldenChestplate => "minecraft:golden_chestplate",
            ItemKind::GoldenLeggings => "minecraft:golden_leggings",
            ItemKind::GoldenBoots => "minecraft:golden_boots",
            ItemKind::NetheriteHelmet => "minecraft:netherite_helmet",
            ItemKind::NetheriteChestplate => "minecraft:netherite_chestplate",
            ItemKind::NetheriteLeggings => "minecraft:netherite_leggings",
            ItemKind::NetheriteBoots => "minecraft:netherite_boots",
            ItemKind::Flint => "minecraft:flint",
            ItemKind::Porkchop => "minecraft:porkchop",
            ItemKind::CookedPorkchop => "minecraft:cooked_porkchop",
            ItemKind::Painting => "minecraft:painting",
            ItemKind::GoldenApple => "minecraft:golden_apple",
            ItemKind::EnchantedGoldenApple => "minecraft:enchanted_golden_apple",
            ItemKind::OakSign => "minecraft:oak_sign",
            ItemKind::SpruceSign => "minecraft:spruce_sign",
            ItemKind::BirchSign => "minecraft:birch_sign",
            ItemKind::JungleSign => "minecraft:jungle_sign",
            ItemKind::AcaciaSign => "minecraft:acacia_sign",
            ItemKind::DarkOakSign => "minecraft:dark_oak_sign",
            ItemKind::CrimsonSign => "minecraft:crimson_sign",
            ItemKind::WarpedSign => "minecraft:warped_sign",
            ItemKind::Bucket => "minecraft:bucket",
            ItemKind::WaterBucket => "minecraft:water_bucket",
            ItemKind::LavaBucket => "minecraft:lava_bucket",
            ItemKind::PowderSnowBucket => "minecraft:powder_snow_bucket",
            ItemKind::Snowball => "minecraft:snowball",
            ItemKind::Leather => "minecraft:leather",
            ItemKind::MilkBucket => "minecraft:milk_bucket",
            ItemKind::PufferfishBucket => "minecraft:pufferfish_bucket",
            ItemKind::SalmonBucket => "minecraft:salmon_bucket",
            ItemKind::CodBucket => "minecraft:cod_bucket",
            ItemKind::TropicalFishBucket => "minecraft:tropical_fish_bucket",
            ItemKind::AxolotlBucket => "minecraft:axolotl_bucket",
            ItemKind::Brick => "minecraft:brick",
            ItemKind::ClayBall => "minecraft:clay_ball",
            ItemKind::DriedKelpBlock => "minecraft:dried_kelp_block",
            ItemKind::Paper => "minecraft:paper",
            ItemKind::Book => "minecraft:book",
            ItemKind::SlimeBall => "minecraft:slime_ball",
            ItemKind::Egg => "minecraft:egg",
            ItemKind::Compass => "minecraft:compass",
            ItemKind::Bundle => "minecraft:bundle",
            ItemKind::FishingRod => "minecraft:fishing_rod",
            ItemKind::Clock => "minecraft:clock",
            ItemKind::Spyglass => "minecraft:spyglass",
            ItemKind::GlowstoneDust => "minecraft:glowstone_dust",
            ItemKind::Cod => "minecraft:cod",
            ItemKind::Salmon => "minecraft:salmon",
            ItemKind::TropicalFish => "minecraft:tropical_fish",
            ItemKind::Pufferfish => "minecraft:pufferfish",
            ItemKind::CookedCod => "minecraft:cooked_cod",
            ItemKind::CookedSalmon => "minecraft:cooked_salmon",
            ItemKind::InkSac => "minecraft:ink_sac",
            ItemKind::GlowInkSac => "minecraft:glow_ink_sac",
            ItemKind::CocoaBeans => "minecraft:cocoa_beans",
            ItemKind::WhiteDye => "minecraft:white_dye",
            ItemKind::OrangeDye => "minecraft:orange_dye",
            ItemKind::MagentaDye => "minecraft:magenta_dye",
            ItemKind::LightBlueDye => "minecraft:light_blue_dye",
            ItemKind::YellowDye => "minecraft:yellow_dye",
            ItemKind::LimeDye => "minecraft:lime_dye",
            ItemKind::PinkDye => "minecraft:pink_dye",
            ItemKind::GrayDye => "minecraft:gray_dye",
            ItemKind::LightGrayDye => "minecraft:light_gray_dye",
            ItemKind::CyanDye => "minecraft:cyan_dye",
            ItemKind::PurpleDye => "minecraft:purple_dye",
            ItemKind::BlueDye => "minecraft:blue_dye",
            ItemKind::BrownDye => "minecraft:brown_dye",
            ItemKind::GreenDye => "minecraft:green_dye",
            ItemKind::RedDye => "minecraft:red_dye",
            ItemKind::BlackDye => "minecraft:black_dye",
            ItemKind::BoneMeal => "minecraft:bone_meal",
            ItemKind::Bone => "minecraft:bone",
            ItemKind::Sugar => "minecraft:sugar",
            ItemKind::Cake => "minecraft:cake",
            ItemKind::WhiteBed => "minecraft:white_bed",
            ItemKind::OrangeBed => "minecraft:orange_bed",
            ItemKind::MagentaBed => "minecraft:magenta_bed",
            ItemKind::LightBlueBed => "minecraft:light_blue_bed",
            ItemKind::YellowBed => "minecraft:yellow_bed",
            ItemKind::LimeBed => "minecraft:lime_bed",
            ItemKind::PinkBed => "minecraft:pink_bed",
            ItemKind::GrayBed => "minecraft:gray_bed",
            ItemKind::LightGrayBed => "minecraft:light_gray_bed",
            ItemKind::CyanBed => "minecraft:cyan_bed",
            ItemKind::PurpleBed => "minecraft:purple_bed",
            ItemKind::BlueBed => "minecraft:blue_bed",
            ItemKind::BrownBed => "minecraft:brown_bed",
            ItemKind::GreenBed => "minecraft:green_bed",
            ItemKind::RedBed => "minecraft:red_bed",
            ItemKind::BlackBed => "minecraft:black_bed",
            ItemKind::Cookie => "minecraft:cookie",
            ItemKind::FilledMap => "minecraft:filled_map",
            ItemKind::Shears => "minecraft:shears",
            ItemKind::MelonSlice => "minecraft:melon_slice",
            ItemKind::DriedKelp => "minecraft:dried_kelp",
            ItemKind::PumpkinSeeds => "minecraft:pumpkin_seeds",
            ItemKind::MelonSeeds => "minecraft:melon_seeds",
            ItemKind::Beef => "minecraft:beef",
            ItemKind::CookedBeef => "minecraft:cooked_beef",
            ItemKind::Chicken => "minecraft:chicken",
            ItemKind::CookedChicken => "minecraft:cooked_chicken",
            ItemKind::RottenFlesh => "minecraft:rotten_flesh",
            ItemKind::EnderPearl => "minecraft:ender_pearl",
            ItemKind::BlazeRod => "minecraft:blaze_rod",
            ItemKind::GhastTear => "minecraft:ghast_tear",
            ItemKind::GoldNugget => "minecraft:gold_nugget",
            ItemKind::NetherWart => "minecraft:nether_wart",
            ItemKind::Potion => "minecraft:potion",
            ItemKind::GlassBottle => "minecraft:glass_bottle",
            ItemKind::SpiderEye => "minecraft:spider_eye",
            ItemKind::FermentedSpiderEye => "minecraft:fermented_spider_eye",
            ItemKind::BlazePowder => "minecraft:blaze_powder",
            ItemKind::MagmaCream => "minecraft:magma_cream",
            ItemKind::BrewingStand => "minecraft:brewing_stand",
            ItemKind::Cauldron => "minecraft:cauldron",
            ItemKind::EnderEye => "minecraft:ender_eye",
            ItemKind::GlisteringMelonSlice => "minecraft:glistering_melon_slice",
            ItemKind::AxolotlSpawnEgg => "minecraft:axolotl_spawn_egg",
            ItemKind::BatSpawnEgg => "minecraft:bat_spawn_egg",
            ItemKind::BeeSpawnEgg => "minecraft:bee_spawn_egg",
            ItemKind::BlazeSpawnEgg => "minecraft:blaze_spawn_egg",
            ItemKind::CatSpawnEgg => "minecraft:cat_spawn_egg",
            ItemKind::CaveSpiderSpawnEgg => "minecraft:cave_spider_spawn_egg",
            ItemKind::ChickenSpawnEgg => "minecraft:chicken_spawn_egg",
            ItemKind::CodSpawnEgg => "minecraft:cod_spawn_egg",
            ItemKind::CowSpawnEgg => "minecraft:cow_spawn_egg",
            ItemKind::CreeperSpawnEgg => "minecraft:creeper_spawn_egg",
            ItemKind::DolphinSpawnEgg => "minecraft:dolphin_spawn_egg",
            ItemKind::DonkeySpawnEgg => "minecraft:donkey_spawn_egg",
            ItemKind::DrownedSpawnEgg => "minecraft:drowned_spawn_egg",
            ItemKind::ElderGuardianSpawnEgg => "minecraft:elder_guardian_spawn_egg",
            ItemKind::EndermanSpawnEgg => "minecraft:enderman_spawn_egg",
            ItemKind::EndermiteSpawnEgg => "minecraft:endermite_spawn_egg",
            ItemKind::EvokerSpawnEgg => "minecraft:evoker_spawn_egg",
            ItemKind::FoxSpawnEgg => "minecraft:fox_spawn_egg",
            ItemKind::GhastSpawnEgg => "minecraft:ghast_spawn_egg",
            ItemKind::GlowSquidSpawnEgg => "minecraft:glow_squid_spawn_egg",
            ItemKind::GoatSpawnEgg => "minecraft:goat_spawn_egg",
            ItemKind::GuardianSpawnEgg => "minecraft:guardian_spawn_egg",
            ItemKind::HoglinSpawnEgg => "minecraft:hoglin_spawn_egg",
            ItemKind::HorseSpawnEgg => "minecraft:horse_spawn_egg",
            ItemKind::HuskSpawnEgg => "minecraft:husk_spawn_egg",
            ItemKind::LlamaSpawnEgg => "minecraft:llama_spawn_egg",
            ItemKind::MagmaCubeSpawnEgg => "minecraft:magma_cube_spawn_egg",
            ItemKind::MooshroomSpawnEgg => "minecraft:mooshroom_spawn_egg",
            ItemKind::MuleSpawnEgg => "minecraft:mule_spawn_egg",
            ItemKind::OcelotSpawnEgg => "minecraft:ocelot_spawn_egg",
            ItemKind::PandaSpawnEgg => "minecraft:panda_spawn_egg",
            ItemKind::ParrotSpawnEgg => "minecraft:parrot_spawn_egg",
            ItemKind::PhantomSpawnEgg => "minecraft:phantom_spawn_egg",
            ItemKind::PigSpawnEgg => "minecraft:pig_spawn_egg",
            ItemKind::PiglinSpawnEgg => "minecraft:piglin_spawn_egg",
            ItemKind::PiglinBruteSpawnEgg => "minecraft:piglin_brute_spawn_egg",
            ItemKind::PillagerSpawnEgg => "minecraft:pillager_spawn_egg",
            ItemKind::PolarBearSpawnEgg => "minecraft:polar_bear_spawn_egg",
            ItemKind::PufferfishSpawnEgg => "minecraft:pufferfish_spawn_egg",
            ItemKind::RabbitSpawnEgg => "minecraft:rabbit_spawn_egg",
            ItemKind::RavagerSpawnEgg => "minecraft:ravager_spawn_egg",
            ItemKind::SalmonSpawnEgg => "minecraft:salmon_spawn_egg",
            ItemKind::SheepSpawnEgg => "minecraft:sheep_spawn_egg",
            ItemKind::ShulkerSpawnEgg => "minecraft:shulker_spawn_egg",
            ItemKind::SilverfishSpawnEgg => "minecraft:silverfish_spawn_egg",
            ItemKind::SkeletonSpawnEgg => "minecraft:skeleton_spawn_egg",
            ItemKind::SkeletonHorseSpawnEgg => "minecraft:skeleton_horse_spawn_egg",
            ItemKind::SlimeSpawnEgg => "minecraft:slime_spawn_egg",
            ItemKind::SpiderSpawnEgg => "minecraft:spider_spawn_egg",
            ItemKind::SquidSpawnEgg => "minecraft:squid_spawn_egg",
            ItemKind::StraySpawnEgg => "minecraft:stray_spawn_egg",
            ItemKind::StriderSpawnEgg => "minecraft:strider_spawn_egg",
            ItemKind::TraderLlamaSpawnEgg => "minecraft:trader_llama_spawn_egg",
            ItemKind::TropicalFishSpawnEgg => "minecraft:tropical_fish_spawn_egg",
            ItemKind::TurtleSpawnEgg => "minecraft:turtle_spawn_egg",
            ItemKind::VexSpawnEgg => "minecraft:vex_spawn_egg",
            ItemKind::VillagerSpawnEgg => "minecraft:villager_spawn_egg",
            ItemKind::VindicatorSpawnEgg => "minecraft:vindicator_spawn_egg",
            ItemKind::WanderingTraderSpawnEgg => "minecraft:wandering_trader_spawn_egg",
            ItemKind::WitchSpawnEgg => "minecraft:witch_spawn_egg",
            ItemKind::WitherSkeletonSpawnEgg => "minecraft:wither_skeleton_spawn_egg",
            ItemKind::WolfSpawnEgg => "minecraft:wolf_spawn_egg",
            ItemKind::ZoglinSpawnEgg => "minecraft:zoglin_spawn_egg",
            ItemKind::ZombieSpawnEgg => "minecraft:zombie_spawn_egg",
            ItemKind::ZombieHorseSpawnEgg => "minecraft:zombie_horse_spawn_egg",
            ItemKind::ZombieVillagerSpawnEgg => "minecraft:zombie_villager_spawn_egg",
            ItemKind::ZombifiedPiglinSpawnEgg => "minecraft:zombified_piglin_spawn_egg",
            ItemKind::ExperienceBottle => "minecraft:experience_bottle",
            ItemKind::FireCharge => "minecraft:fire_charge",
            ItemKind::WritableBook => "minecraft:writable_book",
            ItemKind::WrittenBook => "minecraft:written_book",
            ItemKind::ItemFrame => "minecraft:item_frame",
            ItemKind::GlowItemFrame => "minecraft:glow_item_frame",
            ItemKind::FlowerPot => "minecraft:flower_pot",
            ItemKind::Carrot => "minecraft:carrot",
            ItemKind::Potato => "minecraft:potato",
            ItemKind::BakedPotato => "minecraft:baked_potato",
            ItemKind::PoisonousPotato => "minecraft:poisonous_potato",
            ItemKind::Map => "minecraft:map",
            ItemKind::GoldenCarrot => "minecraft:golden_carrot",
            ItemKind::SkeletonSkull => "minecraft:skeleton_skull",
            ItemKind::WitherSkeletonSkull => "minecraft:wither_skeleton_skull",
            ItemKind::PlayerHead => "minecraft:player_head",
            ItemKind::ZombieHead => "minecraft:zombie_head",
            ItemKind::CreeperHead => "minecraft:creeper_head",
            ItemKind::DragonHead => "minecraft:dragon_head",
            ItemKind::NetherStar => "minecraft:nether_star",
            ItemKind::PumpkinPie => "minecraft:pumpkin_pie",
            ItemKind::FireworkRocket => "minecraft:firework_rocket",
            ItemKind::FireworkStar => "minecraft:firework_star",
            ItemKind::EnchantedBook => "minecraft:enchanted_book",
            ItemKind::NetherBrick => "minecraft:nether_brick",
            ItemKind::PrismarineShard => "minecraft:prismarine_shard",
            ItemKind::PrismarineCrystals => "minecraft:prismarine_crystals",
            ItemKind::Rabbit => "minecraft:rabbit",
            ItemKind::CookedRabbit => "minecraft:cooked_rabbit",
            ItemKind::RabbitStew => "minecraft:rabbit_stew",
            ItemKind::RabbitFoot => "minecraft:rabbit_foot",
            ItemKind::RabbitHide => "minecraft:rabbit_hide",
            ItemKind::ArmorStand => "minecraft:armor_stand",
            ItemKind::IronHorseArmor => "minecraft:iron_horse_armor",
            ItemKind::GoldenHorseArmor => "minecraft:golden_horse_armor",
            ItemKind::DiamondHorseArmor => "minecraft:diamond_horse_armor",
            ItemKind::LeatherHorseArmor => "minecraft:leather_horse_armor",
            ItemKind::Lead => "minecraft:lead",
            ItemKind::NameTag => "minecraft:name_tag",
            ItemKind::CommandBlockMinecart => "minecraft:command_block_minecart",
            ItemKind::Mutton => "minecraft:mutton",
            ItemKind::CookedMutton => "minecraft:cooked_mutton",
            ItemKind::WhiteBanner => "minecraft:white_banner",
            ItemKind::OrangeBanner => "minecraft:orange_banner",
            ItemKind::MagentaBanner => "minecraft:magenta_banner",
            ItemKind::LightBlueBanner => "minecraft:light_blue_banner",
            ItemKind::YellowBanner => "minecraft:yellow_banner",
            ItemKind::LimeBanner => "minecraft:lime_banner",
            ItemKind::PinkBanner => "minecraft:pink_banner",
            ItemKind::GrayBanner => "minecraft:gray_banner",
            ItemKind::LightGrayBanner => "minecraft:light_gray_banner",
            ItemKind::CyanBanner => "minecraft:cyan_banner",
            ItemKind::PurpleBanner => "minecraft:purple_banner",
            ItemKind::BlueBanner => "minecraft:blue_banner",
            ItemKind::BrownBanner => "minecraft:brown_banner",
            ItemKind::GreenBanner => "minecraft:green_banner",
            ItemKind::RedBanner => "minecraft:red_banner",
            ItemKind::BlackBanner => "minecraft:black_banner",
            ItemKind::EndCrystal => "minecraft:end_crystal",
            ItemKind::ChorusFruit => "minecraft:chorus_fruit",
            ItemKind::PoppedChorusFruit => "minecraft:popped_chorus_fruit",
            ItemKind::Beetroot => "minecraft:beetroot",
            ItemKind::BeetrootSeeds => "minecraft:beetroot_seeds",
            ItemKind::BeetrootSoup => "minecraft:beetroot_soup",
            ItemKind::DragonBreath => "minecraft:dragon_breath",
            ItemKind::SplashPotion => "minecraft:splash_potion",
            ItemKind::SpectralArrow => "minecraft:spectral_arrow",
            ItemKind::TippedArrow => "minecraft:tipped_arrow",
            ItemKind::LingeringPotion => "minecraft:lingering_potion",
            ItemKind::Shield => "minecraft:shield",
            ItemKind::TotemOfUndying => "minecraft:totem_of_undying",
            ItemKind::ShulkerShell => "minecraft:shulker_shell",
            ItemKind::IronNugget => "minecraft:iron_nugget",
            ItemKind::KnowledgeBook => "minecraft:knowledge_book",
            ItemKind::DebugStick => "minecraft:debug_stick",
            ItemKind::MusicDisc13 => "minecraft:music_disc_13",
            ItemKind::MusicDiscCat => "minecraft:music_disc_cat",
            ItemKind::MusicDiscBlocks => "minecraft:music_disc_blocks",
            ItemKind::MusicDiscChirp => "minecraft:music_disc_chirp",
            ItemKind::MusicDiscFar => "minecraft:music_disc_far",
            ItemKind::MusicDiscMall => "minecraft:music_disc_mall",
            ItemKind::MusicDiscMellohi => "minecraft:music_disc_mellohi",
            ItemKind::MusicDiscStal => "minecraft:music_disc_stal",
            ItemKind::MusicDiscStrad => "minecraft:music_disc_strad",
            ItemKind::MusicDiscWard => "minecraft:music_disc_ward",
            ItemKind::MusicDisc11 => "minecraft:music_disc_11",
            ItemKind::MusicDiscWait => "minecraft:music_disc_wait",
            ItemKind::MusicDiscOtherside => "minecraft:music_disc_otherside",
            ItemKind::MusicDiscPigstep => "minecraft:music_disc_pigstep",
            ItemKind::Trident => "minecraft:trident",
            ItemKind::PhantomMembrane => "minecraft:phantom_membrane",
            ItemKind::NautilusShell => "minecraft:nautilus_shell",
            ItemKind::HeartOfTheSea => "minecraft:heart_of_the_sea",
            ItemKind::Crossbow => "minecraft:crossbow",
            ItemKind::SuspiciousStew => "minecraft:suspicious_stew",
            ItemKind::Loom => "minecraft:loom",
            ItemKind::FlowerBannerPattern => "minecraft:flower_banner_pattern",
            ItemKind::CreeperBannerPattern => "minecraft:creeper_banner_pattern",
            ItemKind::SkullBannerPattern => "minecraft:skull_banner_pattern",
            ItemKind::MojangBannerPattern => "minecraft:mojang_banner_pattern",
            ItemKind::GlobeBannerPattern => "minecraft:globe_banner_pattern",
            ItemKind::PiglinBannerPattern => "minecraft:piglin_banner_pattern",
            ItemKind::Composter => "minecraft:composter",
            ItemKind::Barrel => "minecraft:barrel",
            ItemKind::Smoker => "minecraft:smoker",
            ItemKind::BlastFurnace => "minecraft:blast_furnace",
            ItemKind::CartographyTable => "minecraft:cartography_table",
            ItemKind::FletchingTable => "minecraft:fletching_table",
            ItemKind::Grindstone => "minecraft:grindstone",
            ItemKind::SmithingTable => "minecraft:smithing_table",
            ItemKind::Stonecutter => "minecraft:stonecutter",
            ItemKind::Bell => "minecraft:bell",
            ItemKind::Lantern => "minecraft:lantern",
            ItemKind::SoulLantern => "minecraft:soul_lantern",
            ItemKind::SweetBerries => "minecraft:sweet_berries",
            ItemKind::GlowBerries => "minecraft:glow_berries",
            ItemKind::Campfire => "minecraft:campfire",
            ItemKind::SoulCampfire => "minecraft:soul_campfire",
            ItemKind::Shroomlight => "minecraft:shroomlight",
            ItemKind::Honeycomb => "minecraft:honeycomb",
            ItemKind::BeeNest => "minecraft:bee_nest",
            ItemKind::Beehive => "minecraft:beehive",
            ItemKind::HoneyBottle => "minecraft:honey_bottle",
            ItemKind::HoneycombBlock => "minecraft:honeycomb_block",
            ItemKind::Lodestone => "minecraft:lodestone",
            ItemKind::CryingObsidian => "minecraft:crying_obsidian",
            ItemKind::Blackstone => "minecraft:blackstone",
            ItemKind::BlackstoneSlab => "minecraft:blackstone_slab",
            ItemKind::BlackstoneStairs => "minecraft:blackstone_stairs",
            ItemKind::GildedBlackstone => "minecraft:gilded_blackstone",
            ItemKind::PolishedBlackstone => "minecraft:polished_blackstone",
            ItemKind::PolishedBlackstoneSlab => "minecraft:polished_blackstone_slab",
            ItemKind::PolishedBlackstoneStairs => "minecraft:polished_blackstone_stairs",
            ItemKind::ChiseledPolishedBlackstone => "minecraft:chiseled_polished_blackstone",
            ItemKind::PolishedBlackstoneBricks => "minecraft:polished_blackstone_bricks",
            ItemKind::PolishedBlackstoneBrickSlab => "minecraft:polished_blackstone_brick_slab",
            ItemKind::PolishedBlackstoneBrickStairs => "minecraft:polished_blackstone_brick_stairs",
            ItemKind::CrackedPolishedBlackstoneBricks => {
                "minecraft:cracked_polished_blackstone_bricks"
            }
            ItemKind::RespawnAnchor => "minecraft:respawn_anchor",
            ItemKind::Candle => "minecraft:candle",
            ItemKind::WhiteCandle => "minecraft:white_candle",
            ItemKind::OrangeCandle => "minecraft:orange_candle",
            ItemKind::MagentaCandle => "minecraft:magenta_candle",
            ItemKind::LightBlueCandle => "minecraft:light_blue_candle",
            ItemKind::YellowCandle => "minecraft:yellow_candle",
            ItemKind::LimeCandle => "minecraft:lime_candle",
            ItemKind::PinkCandle => "minecraft:pink_candle",
            ItemKind::GrayCandle => "minecraft:gray_candle",
            ItemKind::LightGrayCandle => "minecraft:light_gray_candle",
            ItemKind::CyanCandle => "minecraft:cyan_candle",
            ItemKind::PurpleCandle => "minecraft:purple_candle",
            ItemKind::BlueCandle => "minecraft:blue_candle",
            ItemKind::BrownCandle => "minecraft:brown_candle",
            ItemKind::GreenCandle => "minecraft:green_candle",
            ItemKind::RedCandle => "minecraft:red_candle",
            ItemKind::BlackCandle => "minecraft:black_candle",
            ItemKind::SmallAmethystBud => "minecraft:small_amethyst_bud",
            ItemKind::MediumAmethystBud => "minecraft:medium_amethyst_bud",
            ItemKind::LargeAmethystBud => "minecraft:large_amethyst_bud",
            ItemKind::AmethystCluster => "minecraft:amethyst_cluster",
            ItemKind::PointedDripstone => "minecraft:pointed_dripstone",
        }
    }
    #[doc = "Gets a `ItemKind` by its `namespaced_id`."]
    #[inline]
    pub fn from_namespaced_id(namespaced_id: &str) -> Option<Self> {
        match namespaced_id {
            "minecraft:stone" => Some(ItemKind::Stone),
            "minecraft:granite" => Some(ItemKind::Granite),
            "minecraft:polished_granite" => Some(ItemKind::PolishedGranite),
            "minecraft:diorite" => Some(ItemKind::Diorite),
            "minecraft:polished_diorite" => Some(ItemKind::PolishedDiorite),
            "minecraft:andesite" => Some(ItemKind::Andesite),
            "minecraft:polished_andesite" => Some(ItemKind::PolishedAndesite),
            "minecraft:deepslate" => Some(ItemKind::Deepslate),
            "minecraft:cobbled_deepslate" => Some(ItemKind::CobbledDeepslate),
            "minecraft:polished_deepslate" => Some(ItemKind::PolishedDeepslate),
            "minecraft:calcite" => Some(ItemKind::Calcite),
            "minecraft:tuff" => Some(ItemKind::Tuff),
            "minecraft:dripstone_block" => Some(ItemKind::DripstoneBlock),
            "minecraft:grass_block" => Some(ItemKind::GrassBlock),
            "minecraft:dirt" => Some(ItemKind::Dirt),
            "minecraft:coarse_dirt" => Some(ItemKind::CoarseDirt),
            "minecraft:podzol" => Some(ItemKind::Podzol),
            "minecraft:rooted_dirt" => Some(ItemKind::RootedDirt),
            "minecraft:crimson_nylium" => Some(ItemKind::CrimsonNylium),
            "minecraft:warped_nylium" => Some(ItemKind::WarpedNylium),
            "minecraft:cobblestone" => Some(ItemKind::Cobblestone),
            "minecraft:oak_planks" => Some(ItemKind::OakPlanks),
            "minecraft:spruce_planks" => Some(ItemKind::SprucePlanks),
            "minecraft:birch_planks" => Some(ItemKind::BirchPlanks),
            "minecraft:jungle_planks" => Some(ItemKind::JunglePlanks),
            "minecraft:acacia_planks" => Some(ItemKind::AcaciaPlanks),
            "minecraft:dark_oak_planks" => Some(ItemKind::DarkOakPlanks),
            "minecraft:crimson_planks" => Some(ItemKind::CrimsonPlanks),
            "minecraft:warped_planks" => Some(ItemKind::WarpedPlanks),
            "minecraft:oak_sapling" => Some(ItemKind::OakSapling),
            "minecraft:spruce_sapling" => Some(ItemKind::SpruceSapling),
            "minecraft:birch_sapling" => Some(ItemKind::BirchSapling),
            "minecraft:jungle_sapling" => Some(ItemKind::JungleSapling),
            "minecraft:acacia_sapling" => Some(ItemKind::AcaciaSapling),
            "minecraft:dark_oak_sapling" => Some(ItemKind::DarkOakSapling),
            "minecraft:bedrock" => Some(ItemKind::Bedrock),
            "minecraft:sand" => Some(ItemKind::Sand),
            "minecraft:red_sand" => Some(ItemKind::RedSand),
            "minecraft:gravel" => Some(ItemKind::Gravel),
            "minecraft:coal_ore" => Some(ItemKind::CoalOre),
            "minecraft:deepslate_coal_ore" => Some(ItemKind::DeepslateCoalOre),
            "minecraft:iron_ore" => Some(ItemKind::IronOre),
            "minecraft:deepslate_iron_ore" => Some(ItemKind::DeepslateIronOre),
            "minecraft:copper_ore" => Some(ItemKind::CopperOre),
            "minecraft:deepslate_copper_ore" => Some(ItemKind::DeepslateCopperOre),
            "minecraft:gold_ore" => Some(ItemKind::GoldOre),
            "minecraft:deepslate_gold_ore" => Some(ItemKind::DeepslateGoldOre),
            "minecraft:redstone_ore" => Some(ItemKind::RedstoneOre),
            "minecraft:deepslate_redstone_ore" => Some(ItemKind::DeepslateRedstoneOre),
            "minecraft:emerald_ore" => Some(ItemKind::EmeraldOre),
            "minecraft:deepslate_emerald_ore" => Some(ItemKind::DeepslateEmeraldOre),
            "minecraft:lapis_ore" => Some(ItemKind::LapisOre),
            "minecraft:deepslate_lapis_ore" => Some(ItemKind::DeepslateLapisOre),
            "minecraft:diamond_ore" => Some(ItemKind::DiamondOre),
            "minecraft:deepslate_diamond_ore" => Some(ItemKind::DeepslateDiamondOre),
            "minecraft:nether_gold_ore" => Some(ItemKind::NetherGoldOre),
            "minecraft:nether_quartz_ore" => Some(ItemKind::NetherQuartzOre),
            "minecraft:ancient_debris" => Some(ItemKind::AncientDebris),
            "minecraft:coal_block" => Some(ItemKind::CoalBlock),
            "minecraft:raw_iron_block" => Some(ItemKind::RawIronBlock),
            "minecraft:raw_copper_block" => Some(ItemKind::RawCopperBlock),
            "minecraft:raw_gold_block" => Some(ItemKind::RawGoldBlock),
            "minecraft:amethyst_block" => Some(ItemKind::AmethystBlock),
            "minecraft:budding_amethyst" => Some(ItemKind::BuddingAmethyst),
            "minecraft:iron_block" => Some(ItemKind::IronBlock),
            "minecraft:copper_block" => Some(ItemKind::CopperBlock),
            "minecraft:gold_block" => Some(ItemKind::GoldBlock),
            "minecraft:diamond_block" => Some(ItemKind::DiamondBlock),
            "minecraft:netherite_block" => Some(ItemKind::NetheriteBlock),
            "minecraft:exposed_copper" => Some(ItemKind::ExposedCopper),
            "minecraft:weathered_copper" => Some(ItemKind::WeatheredCopper),
            "minecraft:oxidized_copper" => Some(ItemKind::OxidizedCopper),
            "minecraft:cut_copper" => Some(ItemKind::CutCopper),
            "minecraft:exposed_cut_copper" => Some(ItemKind::ExposedCutCopper),
            "minecraft:weathered_cut_copper" => Some(ItemKind::WeatheredCutCopper),
            "minecraft:oxidized_cut_copper" => Some(ItemKind::OxidizedCutCopper),
            "minecraft:cut_copper_stairs" => Some(ItemKind::CutCopperStairs),
            "minecraft:exposed_cut_copper_stairs" => Some(ItemKind::ExposedCutCopperStairs),
            "minecraft:weathered_cut_copper_stairs" => Some(ItemKind::WeatheredCutCopperStairs),
            "minecraft:oxidized_cut_copper_stairs" => Some(ItemKind::OxidizedCutCopperStairs),
            "minecraft:cut_copper_slab" => Some(ItemKind::CutCopperSlab),
            "minecraft:exposed_cut_copper_slab" => Some(ItemKind::ExposedCutCopperSlab),
            "minecraft:weathered_cut_copper_slab" => Some(ItemKind::WeatheredCutCopperSlab),
            "minecraft:oxidized_cut_copper_slab" => Some(ItemKind::OxidizedCutCopperSlab),
            "minecraft:waxed_copper_block" => Some(ItemKind::WaxedCopperBlock),
            "minecraft:waxed_exposed_copper" => Some(ItemKind::WaxedExposedCopper),
            "minecraft:waxed_weathered_copper" => Some(ItemKind::WaxedWeatheredCopper),
            "minecraft:waxed_oxidized_copper" => Some(ItemKind::WaxedOxidizedCopper),
            "minecraft:waxed_cut_copper" => Some(ItemKind::WaxedCutCopper),
            "minecraft:waxed_exposed_cut_copper" => Some(ItemKind::WaxedExposedCutCopper),
            "minecraft:waxed_weathered_cut_copper" => Some(ItemKind::WaxedWeatheredCutCopper),
            "minecraft:waxed_oxidized_cut_copper" => Some(ItemKind::WaxedOxidizedCutCopper),
            "minecraft:waxed_cut_copper_stairs" => Some(ItemKind::WaxedCutCopperStairs),
            "minecraft:waxed_exposed_cut_copper_stairs" => {
                Some(ItemKind::WaxedExposedCutCopperStairs)
            }
            "minecraft:waxed_weathered_cut_copper_stairs" => {
                Some(ItemKind::WaxedWeatheredCutCopperStairs)
            }
            "minecraft:waxed_oxidized_cut_copper_stairs" => {
                Some(ItemKind::WaxedOxidizedCutCopperStairs)
            }
            "minecraft:waxed_cut_copper_slab" => Some(ItemKind::WaxedCutCopperSlab),
            "minecraft:waxed_exposed_cut_copper_slab" => Some(ItemKind::WaxedExposedCutCopperSlab),
            "minecraft:waxed_weathered_cut_copper_slab" => {
                Some(ItemKind::WaxedWeatheredCutCopperSlab)
            }
            "minecraft:waxed_oxidized_cut_copper_slab" => {
                Some(ItemKind::WaxedOxidizedCutCopperSlab)
            }
            "minecraft:oak_log" => Some(ItemKind::OakLog),
            "minecraft:spruce_log" => Some(ItemKind::SpruceLog),
            "minecraft:birch_log" => Some(ItemKind::BirchLog),
            "minecraft:jungle_log" => Some(ItemKind::JungleLog),
            "minecraft:acacia_log" => Some(ItemKind::AcaciaLog),
            "minecraft:dark_oak_log" => Some(ItemKind::DarkOakLog),
            "minecraft:crimson_stem" => Some(ItemKind::CrimsonStem),
            "minecraft:warped_stem" => Some(ItemKind::WarpedStem),
            "minecraft:stripped_oak_log" => Some(ItemKind::StrippedOakLog),
            "minecraft:stripped_spruce_log" => Some(ItemKind::StrippedSpruceLog),
            "minecraft:stripped_birch_log" => Some(ItemKind::StrippedBirchLog),
            "minecraft:stripped_jungle_log" => Some(ItemKind::StrippedJungleLog),
            "minecraft:stripped_acacia_log" => Some(ItemKind::StrippedAcaciaLog),
            "minecraft:stripped_dark_oak_log" => Some(ItemKind::StrippedDarkOakLog),
            "minecraft:stripped_crimson_stem" => Some(ItemKind::StrippedCrimsonStem),
            "minecraft:stripped_warped_stem" => Some(ItemKind::StrippedWarpedStem),
            "minecraft:stripped_oak_wood" => Some(ItemKind::StrippedOakWood),
            "minecraft:stripped_spruce_wood" => Some(ItemKind::StrippedSpruceWood),
            "minecraft:stripped_birch_wood" => Some(ItemKind::StrippedBirchWood),
            "minecraft:stripped_jungle_wood" => Some(ItemKind::StrippedJungleWood),
            "minecraft:stripped_acacia_wood" => Some(ItemKind::StrippedAcaciaWood),
            "minecraft:stripped_dark_oak_wood" => Some(ItemKind::StrippedDarkOakWood),
            "minecraft:stripped_crimson_hyphae" => Some(ItemKind::StrippedCrimsonHyphae),
            "minecraft:stripped_warped_hyphae" => Some(ItemKind::StrippedWarpedHyphae),
            "minecraft:oak_wood" => Some(ItemKind::OakWood),
            "minecraft:spruce_wood" => Some(ItemKind::SpruceWood),
            "minecraft:birch_wood" => Some(ItemKind::BirchWood),
            "minecraft:jungle_wood" => Some(ItemKind::JungleWood),
            "minecraft:acacia_wood" => Some(ItemKind::AcaciaWood),
            "minecraft:dark_oak_wood" => Some(ItemKind::DarkOakWood),
            "minecraft:crimson_hyphae" => Some(ItemKind::CrimsonHyphae),
            "minecraft:warped_hyphae" => Some(ItemKind::WarpedHyphae),
            "minecraft:oak_leaves" => Some(ItemKind::OakLeaves),
            "minecraft:spruce_leaves" => Some(ItemKind::SpruceLeaves),
            "minecraft:birch_leaves" => Some(ItemKind::BirchLeaves),
            "minecraft:jungle_leaves" => Some(ItemKind::JungleLeaves),
            "minecraft:acacia_leaves" => Some(ItemKind::AcaciaLeaves),
            "minecraft:dark_oak_leaves" => Some(ItemKind::DarkOakLeaves),
            "minecraft:azalea_leaves" => Some(ItemKind::AzaleaLeaves),
            "minecraft:flowering_azalea_leaves" => Some(ItemKind::FloweringAzaleaLeaves),
            "minecraft:sponge" => Some(ItemKind::Sponge),
            "minecraft:wet_sponge" => Some(ItemKind::WetSponge),
            "minecraft:glass" => Some(ItemKind::Glass),
            "minecraft:tinted_glass" => Some(ItemKind::TintedGlass),
            "minecraft:lapis_block" => Some(ItemKind::LapisBlock),
            "minecraft:sandstone" => Some(ItemKind::Sandstone),
            "minecraft:chiseled_sandstone" => Some(ItemKind::ChiseledSandstone),
            "minecraft:cut_sandstone" => Some(ItemKind::CutSandstone),
            "minecraft:cobweb" => Some(ItemKind::Cobweb),
            "minecraft:grass" => Some(ItemKind::Grass),
            "minecraft:fern" => Some(ItemKind::Fern),
            "minecraft:azalea" => Some(ItemKind::Azalea),
            "minecraft:flowering_azalea" => Some(ItemKind::FloweringAzalea),
            "minecraft:dead_bush" => Some(ItemKind::DeadBush),
            "minecraft:seagrass" => Some(ItemKind::Seagrass),
            "minecraft:sea_pickle" => Some(ItemKind::SeaPickle),
            "minecraft:white_wool" => Some(ItemKind::WhiteWool),
            "minecraft:orange_wool" => Some(ItemKind::OrangeWool),
            "minecraft:magenta_wool" => Some(ItemKind::MagentaWool),
            "minecraft:light_blue_wool" => Some(ItemKind::LightBlueWool),
            "minecraft:yellow_wool" => Some(ItemKind::YellowWool),
            "minecraft:lime_wool" => Some(ItemKind::LimeWool),
            "minecraft:pink_wool" => Some(ItemKind::PinkWool),
            "minecraft:gray_wool" => Some(ItemKind::GrayWool),
            "minecraft:light_gray_wool" => Some(ItemKind::LightGrayWool),
            "minecraft:cyan_wool" => Some(ItemKind::CyanWool),
            "minecraft:purple_wool" => Some(ItemKind::PurpleWool),
            "minecraft:blue_wool" => Some(ItemKind::BlueWool),
            "minecraft:brown_wool" => Some(ItemKind::BrownWool),
            "minecraft:green_wool" => Some(ItemKind::GreenWool),
            "minecraft:red_wool" => Some(ItemKind::RedWool),
            "minecraft:black_wool" => Some(ItemKind::BlackWool),
            "minecraft:dandelion" => Some(ItemKind::Dandelion),
            "minecraft:poppy" => Some(ItemKind::Poppy),
            "minecraft:blue_orchid" => Some(ItemKind::BlueOrchid),
            "minecraft:allium" => Some(ItemKind::Allium),
            "minecraft:azure_bluet" => Some(ItemKind::AzureBluet),
            "minecraft:red_tulip" => Some(ItemKind::RedTulip),
            "minecraft:orange_tulip" => Some(ItemKind::OrangeTulip),
            "minecraft:white_tulip" => Some(ItemKind::WhiteTulip),
            "minecraft:pink_tulip" => Some(ItemKind::PinkTulip),
            "minecraft:oxeye_daisy" => Some(ItemKind::OxeyeDaisy),
            "minecraft:cornflower" => Some(ItemKind::Cornflower),
            "minecraft:lily_of_the_valley" => Some(ItemKind::LilyOfTheValley),
            "minecraft:wither_rose" => Some(ItemKind::WitherRose),
            "minecraft:spore_blossom" => Some(ItemKind::SporeBlossom),
            "minecraft:brown_mushroom" => Some(ItemKind::BrownMushroom),
            "minecraft:red_mushroom" => Some(ItemKind::RedMushroom),
            "minecraft:crimson_fungus" => Some(ItemKind::CrimsonFungus),
            "minecraft:warped_fungus" => Some(ItemKind::WarpedFungus),
            "minecraft:crimson_roots" => Some(ItemKind::CrimsonRoots),
            "minecraft:warped_roots" => Some(ItemKind::WarpedRoots),
            "minecraft:nether_sprouts" => Some(ItemKind::NetherSprouts),
            "minecraft:weeping_vines" => Some(ItemKind::WeepingVines),
            "minecraft:twisting_vines" => Some(ItemKind::TwistingVines),
            "minecraft:sugar_cane" => Some(ItemKind::SugarCane),
            "minecraft:kelp" => Some(ItemKind::Kelp),
            "minecraft:moss_carpet" => Some(ItemKind::MossCarpet),
            "minecraft:moss_block" => Some(ItemKind::MossBlock),
            "minecraft:hanging_roots" => Some(ItemKind::HangingRoots),
            "minecraft:big_dripleaf" => Some(ItemKind::BigDripleaf),
            "minecraft:small_dripleaf" => Some(ItemKind::SmallDripleaf),
            "minecraft:bamboo" => Some(ItemKind::Bamboo),
            "minecraft:oak_slab" => Some(ItemKind::OakSlab),
            "minecraft:spruce_slab" => Some(ItemKind::SpruceSlab),
            "minecraft:birch_slab" => Some(ItemKind::BirchSlab),
            "minecraft:jungle_slab" => Some(ItemKind::JungleSlab),
            "minecraft:acacia_slab" => Some(ItemKind::AcaciaSlab),
            "minecraft:dark_oak_slab" => Some(ItemKind::DarkOakSlab),
            "minecraft:crimson_slab" => Some(ItemKind::CrimsonSlab),
            "minecraft:warped_slab" => Some(ItemKind::WarpedSlab),
            "minecraft:stone_slab" => Some(ItemKind::StoneSlab),
            "minecraft:smooth_stone_slab" => Some(ItemKind::SmoothStoneSlab),
            "minecraft:sandstone_slab" => Some(ItemKind::SandstoneSlab),
            "minecraft:cut_sandstone_slab" => Some(ItemKind::CutSandstoneSlab),
            "minecraft:petrified_oak_slab" => Some(ItemKind::PetrifiedOakSlab),
            "minecraft:cobblestone_slab" => Some(ItemKind::CobblestoneSlab),
            "minecraft:brick_slab" => Some(ItemKind::BrickSlab),
            "minecraft:stone_brick_slab" => Some(ItemKind::StoneBrickSlab),
            "minecraft:nether_brick_slab" => Some(ItemKind::NetherBrickSlab),
            "minecraft:quartz_slab" => Some(ItemKind::QuartzSlab),
            "minecraft:red_sandstone_slab" => Some(ItemKind::RedSandstoneSlab),
            "minecraft:cut_red_sandstone_slab" => Some(ItemKind::CutRedSandstoneSlab),
            "minecraft:purpur_slab" => Some(ItemKind::PurpurSlab),
            "minecraft:prismarine_slab" => Some(ItemKind::PrismarineSlab),
            "minecraft:prismarine_brick_slab" => Some(ItemKind::PrismarineBrickSlab),
            "minecraft:dark_prismarine_slab" => Some(ItemKind::DarkPrismarineSlab),
            "minecraft:smooth_quartz" => Some(ItemKind::SmoothQuartz),
            "minecraft:smooth_red_sandstone" => Some(ItemKind::SmoothRedSandstone),
            "minecraft:smooth_sandstone" => Some(ItemKind::SmoothSandstone),
            "minecraft:smooth_stone" => Some(ItemKind::SmoothStone),
            "minecraft:bricks" => Some(ItemKind::Bricks),
            "minecraft:bookshelf" => Some(ItemKind::Bookshelf),
            "minecraft:mossy_cobblestone" => Some(ItemKind::MossyCobblestone),
            "minecraft:obsidian" => Some(ItemKind::Obsidian),
            "minecraft:torch" => Some(ItemKind::Torch),
            "minecraft:end_rod" => Some(ItemKind::EndRod),
            "minecraft:chorus_plant" => Some(ItemKind::ChorusPlant),
            "minecraft:chorus_flower" => Some(ItemKind::ChorusFlower),
            "minecraft:purpur_block" => Some(ItemKind::PurpurBlock),
            "minecraft:purpur_pillar" => Some(ItemKind::PurpurPillar),
            "minecraft:purpur_stairs" => Some(ItemKind::PurpurStairs),
            "minecraft:spawner" => Some(ItemKind::Spawner),
            "minecraft:oak_stairs" => Some(ItemKind::OakStairs),
            "minecraft:chest" => Some(ItemKind::Chest),
            "minecraft:crafting_table" => Some(ItemKind::CraftingTable),
            "minecraft:farmland" => Some(ItemKind::Farmland),
            "minecraft:furnace" => Some(ItemKind::Furnace),
            "minecraft:ladder" => Some(ItemKind::Ladder),
            "minecraft:cobblestone_stairs" => Some(ItemKind::CobblestoneStairs),
            "minecraft:snow" => Some(ItemKind::Snow),
            "minecraft:ice" => Some(ItemKind::Ice),
            "minecraft:snow_block" => Some(ItemKind::SnowBlock),
            "minecraft:cactus" => Some(ItemKind::Cactus),
            "minecraft:clay" => Some(ItemKind::Clay),
            "minecraft:jukebox" => Some(ItemKind::Jukebox),
            "minecraft:oak_fence" => Some(ItemKind::OakFence),
            "minecraft:spruce_fence" => Some(ItemKind::SpruceFence),
            "minecraft:birch_fence" => Some(ItemKind::BirchFence),
            "minecraft:jungle_fence" => Some(ItemKind::JungleFence),
            "minecraft:acacia_fence" => Some(ItemKind::AcaciaFence),
            "minecraft:dark_oak_fence" => Some(ItemKind::DarkOakFence),
            "minecraft:crimson_fence" => Some(ItemKind::CrimsonFence),
            "minecraft:warped_fence" => Some(ItemKind::WarpedFence),
            "minecraft:pumpkin" => Some(ItemKind::Pumpkin),
            "minecraft:carved_pumpkin" => Some(ItemKind::CarvedPumpkin),
            "minecraft:jack_o_lantern" => Some(ItemKind::JackOLantern),
            "minecraft:netherrack" => Some(ItemKind::Netherrack),
            "minecraft:soul_sand" => Some(ItemKind::SoulSand),
            "minecraft:soul_soil" => Some(ItemKind::SoulSoil),
            "minecraft:basalt" => Some(ItemKind::Basalt),
            "minecraft:polished_basalt" => Some(ItemKind::PolishedBasalt),
            "minecraft:smooth_basalt" => Some(ItemKind::SmoothBasalt),
            "minecraft:soul_torch" => Some(ItemKind::SoulTorch),
            "minecraft:glowstone" => Some(ItemKind::Glowstone),
            "minecraft:infested_stone" => Some(ItemKind::InfestedStone),
            "minecraft:infested_cobblestone" => Some(ItemKind::InfestedCobblestone),
            "minecraft:infested_stone_bricks" => Some(ItemKind::InfestedStoneBricks),
            "minecraft:infested_mossy_stone_bricks" => Some(ItemKind::InfestedMossyStoneBricks),
            "minecraft:infested_cracked_stone_bricks" => Some(ItemKind::InfestedCrackedStoneBricks),
            "minecraft:infested_chiseled_stone_bricks" => {
                Some(ItemKind::InfestedChiseledStoneBricks)
            }
            "minecraft:infested_deepslate" => Some(ItemKind::InfestedDeepslate),
            "minecraft:stone_bricks" => Some(ItemKind::StoneBricks),
            "minecraft:mossy_stone_bricks" => Some(ItemKind::MossyStoneBricks),
            "minecraft:cracked_stone_bricks" => Some(ItemKind::CrackedStoneBricks),
            "minecraft:chiseled_stone_bricks" => Some(ItemKind::ChiseledStoneBricks),
            "minecraft:deepslate_bricks" => Some(ItemKind::DeepslateBricks),
            "minecraft:cracked_deepslate_bricks" => Some(ItemKind::CrackedDeepslateBricks),
            "minecraft:deepslate_tiles" => Some(ItemKind::DeepslateTiles),
            "minecraft:cracked_deepslate_tiles" => Some(ItemKind::CrackedDeepslateTiles),
            "minecraft:chiseled_deepslate" => Some(ItemKind::ChiseledDeepslate),
            "minecraft:brown_mushroom_block" => Some(ItemKind::BrownMushroomBlock),
            "minecraft:red_mushroom_block" => Some(ItemKind::RedMushroomBlock),
            "minecraft:mushroom_stem" => Some(ItemKind::MushroomStem),
            "minecraft:iron_bars" => Some(ItemKind::IronBars),
            "minecraft:chain" => Some(ItemKind::Chain),
            "minecraft:glass_pane" => Some(ItemKind::GlassPane),
            "minecraft:melon" => Some(ItemKind::Melon),
            "minecraft:vine" => Some(ItemKind::Vine),
            "minecraft:glow_lichen" => Some(ItemKind::GlowLichen),
            "minecraft:brick_stairs" => Some(ItemKind::BrickStairs),
            "minecraft:stone_brick_stairs" => Some(ItemKind::StoneBrickStairs),
            "minecraft:mycelium" => Some(ItemKind::Mycelium),
            "minecraft:lily_pad" => Some(ItemKind::LilyPad),
            "minecraft:nether_bricks" => Some(ItemKind::NetherBricks),
            "minecraft:cracked_nether_bricks" => Some(ItemKind::CrackedNetherBricks),
            "minecraft:chiseled_nether_bricks" => Some(ItemKind::ChiseledNetherBricks),
            "minecraft:nether_brick_fence" => Some(ItemKind::NetherBrickFence),
            "minecraft:nether_brick_stairs" => Some(ItemKind::NetherBrickStairs),
            "minecraft:enchanting_table" => Some(ItemKind::EnchantingTable),
            "minecraft:end_portal_frame" => Some(ItemKind::EndPortalFrame),
            "minecraft:end_stone" => Some(ItemKind::EndStone),
            "minecraft:end_stone_bricks" => Some(ItemKind::EndStoneBricks),
            "minecraft:dragon_egg" => Some(ItemKind::DragonEgg),
            "minecraft:sandstone_stairs" => Some(ItemKind::SandstoneStairs),
            "minecraft:ender_chest" => Some(ItemKind::EnderChest),
            "minecraft:emerald_block" => Some(ItemKind::EmeraldBlock),
            "minecraft:spruce_stairs" => Some(ItemKind::SpruceStairs),
            "minecraft:birch_stairs" => Some(ItemKind::BirchStairs),
            "minecraft:jungle_stairs" => Some(ItemKind::JungleStairs),
            "minecraft:crimson_stairs" => Some(ItemKind::CrimsonStairs),
            "minecraft:warped_stairs" => Some(ItemKind::WarpedStairs),
            "minecraft:command_block" => Some(ItemKind::CommandBlock),
            "minecraft:beacon" => Some(ItemKind::Beacon),
            "minecraft:cobblestone_wall" => Some(ItemKind::CobblestoneWall),
            "minecraft:mossy_cobblestone_wall" => Some(ItemKind::MossyCobblestoneWall),
            "minecraft:brick_wall" => Some(ItemKind::BrickWall),
            "minecraft:prismarine_wall" => Some(ItemKind::PrismarineWall),
            "minecraft:red_sandstone_wall" => Some(ItemKind::RedSandstoneWall),
            "minecraft:mossy_stone_brick_wall" => Some(ItemKind::MossyStoneBrickWall),
            "minecraft:granite_wall" => Some(ItemKind::GraniteWall),
            "minecraft:stone_brick_wall" => Some(ItemKind::StoneBrickWall),
            "minecraft:nether_brick_wall" => Some(ItemKind::NetherBrickWall),
            "minecraft:andesite_wall" => Some(ItemKind::AndesiteWall),
            "minecraft:red_nether_brick_wall" => Some(ItemKind::RedNetherBrickWall),
            "minecraft:sandstone_wall" => Some(ItemKind::SandstoneWall),
            "minecraft:end_stone_brick_wall" => Some(ItemKind::EndStoneBrickWall),
            "minecraft:diorite_wall" => Some(ItemKind::DioriteWall),
            "minecraft:blackstone_wall" => Some(ItemKind::BlackstoneWall),
            "minecraft:polished_blackstone_wall" => Some(ItemKind::PolishedBlackstoneWall),
            "minecraft:polished_blackstone_brick_wall" => {
                Some(ItemKind::PolishedBlackstoneBrickWall)
            }
            "minecraft:cobbled_deepslate_wall" => Some(ItemKind::CobbledDeepslateWall),
            "minecraft:polished_deepslate_wall" => Some(ItemKind::PolishedDeepslateWall),
            "minecraft:deepslate_brick_wall" => Some(ItemKind::DeepslateBrickWall),
            "minecraft:deepslate_tile_wall" => Some(ItemKind::DeepslateTileWall),
            "minecraft:anvil" => Some(ItemKind::Anvil),
            "minecraft:chipped_anvil" => Some(ItemKind::ChippedAnvil),
            "minecraft:damaged_anvil" => Some(ItemKind::DamagedAnvil),
            "minecraft:chiseled_quartz_block" => Some(ItemKind::ChiseledQuartzBlock),
            "minecraft:quartz_block" => Some(ItemKind::QuartzBlock),
            "minecraft:quartz_bricks" => Some(ItemKind::QuartzBricks),
            "minecraft:quartz_pillar" => Some(ItemKind::QuartzPillar),
            "minecraft:quartz_stairs" => Some(ItemKind::QuartzStairs),
            "minecraft:white_terracotta" => Some(ItemKind::WhiteTerracotta),
            "minecraft:orange_terracotta" => Some(ItemKind::OrangeTerracotta),
            "minecraft:magenta_terracotta" => Some(ItemKind::MagentaTerracotta),
            "minecraft:light_blue_terracotta" => Some(ItemKind::LightBlueTerracotta),
            "minecraft:yellow_terracotta" => Some(ItemKind::YellowTerracotta),
            "minecraft:lime_terracotta" => Some(ItemKind::LimeTerracotta),
            "minecraft:pink_terracotta" => Some(ItemKind::PinkTerracotta),
            "minecraft:gray_terracotta" => Some(ItemKind::GrayTerracotta),
            "minecraft:light_gray_terracotta" => Some(ItemKind::LightGrayTerracotta),
            "minecraft:cyan_terracotta" => Some(ItemKind::CyanTerracotta),
            "minecraft:purple_terracotta" => Some(ItemKind::PurpleTerracotta),
            "minecraft:blue_terracotta" => Some(ItemKind::BlueTerracotta),
            "minecraft:brown_terracotta" => Some(ItemKind::BrownTerracotta),
            "minecraft:green_terracotta" => Some(ItemKind::GreenTerracotta),
            "minecraft:red_terracotta" => Some(ItemKind::RedTerracotta),
            "minecraft:black_terracotta" => Some(ItemKind::BlackTerracotta),
            "minecraft:barrier" => Some(ItemKind::Barrier),
            "minecraft:light" => Some(ItemKind::Light),
            "minecraft:hay_block" => Some(ItemKind::HayBlock),
            "minecraft:white_carpet" => Some(ItemKind::WhiteCarpet),
            "minecraft:orange_carpet" => Some(ItemKind::OrangeCarpet),
            "minecraft:magenta_carpet" => Some(ItemKind::MagentaCarpet),
            "minecraft:light_blue_carpet" => Some(ItemKind::LightBlueCarpet),
            "minecraft:yellow_carpet" => Some(ItemKind::YellowCarpet),
            "minecraft:lime_carpet" => Some(ItemKind::LimeCarpet),
            "minecraft:pink_carpet" => Some(ItemKind::PinkCarpet),
            "minecraft:gray_carpet" => Some(ItemKind::GrayCarpet),
            "minecraft:light_gray_carpet" => Some(ItemKind::LightGrayCarpet),
            "minecraft:cyan_carpet" => Some(ItemKind::CyanCarpet),
            "minecraft:purple_carpet" => Some(ItemKind::PurpleCarpet),
            "minecraft:blue_carpet" => Some(ItemKind::BlueCarpet),
            "minecraft:brown_carpet" => Some(ItemKind::BrownCarpet),
            "minecraft:green_carpet" => Some(ItemKind::GreenCarpet),
            "minecraft:red_carpet" => Some(ItemKind::RedCarpet),
            "minecraft:black_carpet" => Some(ItemKind::BlackCarpet),
            "minecraft:terracotta" => Some(ItemKind::Terracotta),
            "minecraft:packed_ice" => Some(ItemKind::PackedIce),
            "minecraft:acacia_stairs" => Some(ItemKind::AcaciaStairs),
            "minecraft:dark_oak_stairs" => Some(ItemKind::DarkOakStairs),
            "minecraft:dirt_path" => Some(ItemKind::DirtPath),
            "minecraft:sunflower" => Some(ItemKind::Sunflower),
            "minecraft:lilac" => Some(ItemKind::Lilac),
            "minecraft:rose_bush" => Some(ItemKind::RoseBush),
            "minecraft:peony" => Some(ItemKind::Peony),
            "minecraft:tall_grass" => Some(ItemKind::TallGrass),
            "minecraft:large_fern" => Some(ItemKind::LargeFern),
            "minecraft:white_stained_glass" => Some(ItemKind::WhiteStainedGlass),
            "minecraft:orange_stained_glass" => Some(ItemKind::OrangeStainedGlass),
            "minecraft:magenta_stained_glass" => Some(ItemKind::MagentaStainedGlass),
            "minecraft:light_blue_stained_glass" => Some(ItemKind::LightBlueStainedGlass),
            "minecraft:yellow_stained_glass" => Some(ItemKind::YellowStainedGlass),
            "minecraft:lime_stained_glass" => Some(ItemKind::LimeStainedGlass),
            "minecraft:pink_stained_glass" => Some(ItemKind::PinkStainedGlass),
            "minecraft:gray_stained_glass" => Some(ItemKind::GrayStainedGlass),
            "minecraft:light_gray_stained_glass" => Some(ItemKind::LightGrayStainedGlass),
            "minecraft:cyan_stained_glass" => Some(ItemKind::CyanStainedGlass),
            "minecraft:purple_stained_glass" => Some(ItemKind::PurpleStainedGlass),
            "minecraft:blue_stained_glass" => Some(ItemKind::BlueStainedGlass),
            "minecraft:brown_stained_glass" => Some(ItemKind::BrownStainedGlass),
            "minecraft:green_stained_glass" => Some(ItemKind::GreenStainedGlass),
            "minecraft:red_stained_glass" => Some(ItemKind::RedStainedGlass),
            "minecraft:black_stained_glass" => Some(ItemKind::BlackStainedGlass),
            "minecraft:white_stained_glass_pane" => Some(ItemKind::WhiteStainedGlassPane),
            "minecraft:orange_stained_glass_pane" => Some(ItemKind::OrangeStainedGlassPane),
            "minecraft:magenta_stained_glass_pane" => Some(ItemKind::MagentaStainedGlassPane),
            "minecraft:light_blue_stained_glass_pane" => Some(ItemKind::LightBlueStainedGlassPane),
            "minecraft:yellow_stained_glass_pane" => Some(ItemKind::YellowStainedGlassPane),
            "minecraft:lime_stained_glass_pane" => Some(ItemKind::LimeStainedGlassPane),
            "minecraft:pink_stained_glass_pane" => Some(ItemKind::PinkStainedGlassPane),
            "minecraft:gray_stained_glass_pane" => Some(ItemKind::GrayStainedGlassPane),
            "minecraft:light_gray_stained_glass_pane" => Some(ItemKind::LightGrayStainedGlassPane),
            "minecraft:cyan_stained_glass_pane" => Some(ItemKind::CyanStainedGlassPane),
            "minecraft:purple_stained_glass_pane" => Some(ItemKind::PurpleStainedGlassPane),
            "minecraft:blue_stained_glass_pane" => Some(ItemKind::BlueStainedGlassPane),
            "minecraft:brown_stained_glass_pane" => Some(ItemKind::BrownStainedGlassPane),
            "minecraft:green_stained_glass_pane" => Some(ItemKind::GreenStainedGlassPane),
            "minecraft:red_stained_glass_pane" => Some(ItemKind::RedStainedGlassPane),
            "minecraft:black_stained_glass_pane" => Some(ItemKind::BlackStainedGlassPane),
            "minecraft:prismarine" => Some(ItemKind::Prismarine),
            "minecraft:prismarine_bricks" => Some(ItemKind::PrismarineBricks),
            "minecraft:dark_prismarine" => Some(ItemKind::DarkPrismarine),
            "minecraft:prismarine_stairs" => Some(ItemKind::PrismarineStairs),
            "minecraft:prismarine_brick_stairs" => Some(ItemKind::PrismarineBrickStairs),
            "minecraft:dark_prismarine_stairs" => Some(ItemKind::DarkPrismarineStairs),
            "minecraft:sea_lantern" => Some(ItemKind::SeaLantern),
            "minecraft:red_sandstone" => Some(ItemKind::RedSandstone),
            "minecraft:chiseled_red_sandstone" => Some(ItemKind::ChiseledRedSandstone),
            "minecraft:cut_red_sandstone" => Some(ItemKind::CutRedSandstone),
            "minecraft:red_sandstone_stairs" => Some(ItemKind::RedSandstoneStairs),
            "minecraft:repeating_command_block" => Some(ItemKind::RepeatingCommandBlock),
            "minecraft:chain_command_block" => Some(ItemKind::ChainCommandBlock),
            "minecraft:magma_block" => Some(ItemKind::MagmaBlock),
            "minecraft:nether_wart_block" => Some(ItemKind::NetherWartBlock),
            "minecraft:warped_wart_block" => Some(ItemKind::WarpedWartBlock),
            "minecraft:red_nether_bricks" => Some(ItemKind::RedNetherBricks),
            "minecraft:bone_block" => Some(ItemKind::BoneBlock),
            "minecraft:structure_void" => Some(ItemKind::StructureVoid),
            "minecraft:shulker_box" => Some(ItemKind::ShulkerBox),
            "minecraft:white_shulker_box" => Some(ItemKind::WhiteShulkerBox),
            "minecraft:orange_shulker_box" => Some(ItemKind::OrangeShulkerBox),
            "minecraft:magenta_shulker_box" => Some(ItemKind::MagentaShulkerBox),
            "minecraft:light_blue_shulker_box" => Some(ItemKind::LightBlueShulkerBox),
            "minecraft:yellow_shulker_box" => Some(ItemKind::YellowShulkerBox),
            "minecraft:lime_shulker_box" => Some(ItemKind::LimeShulkerBox),
            "minecraft:pink_shulker_box" => Some(ItemKind::PinkShulkerBox),
            "minecraft:gray_shulker_box" => Some(ItemKind::GrayShulkerBox),
            "minecraft:light_gray_shulker_box" => Some(ItemKind::LightGrayShulkerBox),
            "minecraft:cyan_shulker_box" => Some(ItemKind::CyanShulkerBox),
            "minecraft:purple_shulker_box" => Some(ItemKind::PurpleShulkerBox),
            "minecraft:blue_shulker_box" => Some(ItemKind::BlueShulkerBox),
            "minecraft:brown_shulker_box" => Some(ItemKind::BrownShulkerBox),
            "minecraft:green_shulker_box" => Some(ItemKind::GreenShulkerBox),
            "minecraft:red_shulker_box" => Some(ItemKind::RedShulkerBox),
            "minecraft:black_shulker_box" => Some(ItemKind::BlackShulkerBox),
            "minecraft:white_glazed_terracotta" => Some(ItemKind::WhiteGlazedTerracotta),
            "minecraft:orange_glazed_terracotta" => Some(ItemKind::OrangeGlazedTerracotta),
            "minecraft:magenta_glazed_terracotta" => Some(ItemKind::MagentaGlazedTerracotta),
            "minecraft:light_blue_glazed_terracotta" => Some(ItemKind::LightBlueGlazedTerracotta),
            "minecraft:yellow_glazed_terracotta" => Some(ItemKind::YellowGlazedTerracotta),
            "minecraft:lime_glazed_terracotta" => Some(ItemKind::LimeGlazedTerracotta),
            "minecraft:pink_glazed_terracotta" => Some(ItemKind::PinkGlazedTerracotta),
            "minecraft:gray_glazed_terracotta" => Some(ItemKind::GrayGlazedTerracotta),
            "minecraft:light_gray_glazed_terracotta" => Some(ItemKind::LightGrayGlazedTerracotta),
            "minecraft:cyan_glazed_terracotta" => Some(ItemKind::CyanGlazedTerracotta),
            "minecraft:purple_glazed_terracotta" => Some(ItemKind::PurpleGlazedTerracotta),
            "minecraft:blue_glazed_terracotta" => Some(ItemKind::BlueGlazedTerracotta),
            "minecraft:brown_glazed_terracotta" => Some(ItemKind::BrownGlazedTerracotta),
            "minecraft:green_glazed_terracotta" => Some(ItemKind::GreenGlazedTerracotta),
            "minecraft:red_glazed_terracotta" => Some(ItemKind::RedGlazedTerracotta),
            "minecraft:black_glazed_terracotta" => Some(ItemKind::BlackGlazedTerracotta),
            "minecraft:white_concrete" => Some(ItemKind::WhiteConcrete),
            "minecraft:orange_concrete" => Some(ItemKind::OrangeConcrete),
            "minecraft:magenta_concrete" => Some(ItemKind::MagentaConcrete),
            "minecraft:light_blue_concrete" => Some(ItemKind::LightBlueConcrete),
            "minecraft:yellow_concrete" => Some(ItemKind::YellowConcrete),
            "minecraft:lime_concrete" => Some(ItemKind::LimeConcrete),
            "minecraft:pink_concrete" => Some(ItemKind::PinkConcrete),
            "minecraft:gray_concrete" => Some(ItemKind::GrayConcrete),
            "minecraft:light_gray_concrete" => Some(ItemKind::LightGrayConcrete),
            "minecraft:cyan_concrete" => Some(ItemKind::CyanConcrete),
            "minecraft:purple_concrete" => Some(ItemKind::PurpleConcrete),
            "minecraft:blue_concrete" => Some(ItemKind::BlueConcrete),
            "minecraft:brown_concrete" => Some(ItemKind::BrownConcrete),
            "minecraft:green_concrete" => Some(ItemKind::GreenConcrete),
            "minecraft:red_concrete" => Some(ItemKind::RedConcrete),
            "minecraft:black_concrete" => Some(ItemKind::BlackConcrete),
            "minecraft:white_concrete_powder" => Some(ItemKind::WhiteConcretePowder),
            "minecraft:orange_concrete_powder" => Some(ItemKind::OrangeConcretePowder),
            "minecraft:magenta_concrete_powder" => Some(ItemKind::MagentaConcretePowder),
            "minecraft:light_blue_concrete_powder" => Some(ItemKind::LightBlueConcretePowder),
            "minecraft:yellow_concrete_powder" => Some(ItemKind::YellowConcretePowder),
            "minecraft:lime_concrete_powder" => Some(ItemKind::LimeConcretePowder),
            "minecraft:pink_concrete_powder" => Some(ItemKind::PinkConcretePowder),
            "minecraft:gray_concrete_powder" => Some(ItemKind::GrayConcretePowder),
            "minecraft:light_gray_concrete_powder" => Some(ItemKind::LightGrayConcretePowder),
            "minecraft:cyan_concrete_powder" => Some(ItemKind::CyanConcretePowder),
            "minecraft:purple_concrete_powder" => Some(ItemKind::PurpleConcretePowder),
            "minecraft:blue_concrete_powder" => Some(ItemKind::BlueConcretePowder),
            "minecraft:brown_concrete_powder" => Some(ItemKind::BrownConcretePowder),
            "minecraft:green_concrete_powder" => Some(ItemKind::GreenConcretePowder),
            "minecraft:red_concrete_powder" => Some(ItemKind::RedConcretePowder),
            "minecraft:black_concrete_powder" => Some(ItemKind::BlackConcretePowder),
            "minecraft:turtle_egg" => Some(ItemKind::TurtleEgg),
            "minecraft:dead_tube_coral_block" => Some(ItemKind::DeadTubeCoralBlock),
            "minecraft:dead_brain_coral_block" => Some(ItemKind::DeadBrainCoralBlock),
            "minecraft:dead_bubble_coral_block" => Some(ItemKind::DeadBubbleCoralBlock),
            "minecraft:dead_fire_coral_block" => Some(ItemKind::DeadFireCoralBlock),
            "minecraft:dead_horn_coral_block" => Some(ItemKind::DeadHornCoralBlock),
            "minecraft:tube_coral_block" => Some(ItemKind::TubeCoralBlock),
            "minecraft:brain_coral_block" => Some(ItemKind::BrainCoralBlock),
            "minecraft:bubble_coral_block" => Some(ItemKind::BubbleCoralBlock),
            "minecraft:fire_coral_block" => Some(ItemKind::FireCoralBlock),
            "minecraft:horn_coral_block" => Some(ItemKind::HornCoralBlock),
            "minecraft:tube_coral" => Some(ItemKind::TubeCoral),
            "minecraft:brain_coral" => Some(ItemKind::BrainCoral),
            "minecraft:bubble_coral" => Some(ItemKind::BubbleCoral),
            "minecraft:fire_coral" => Some(ItemKind::FireCoral),
            "minecraft:horn_coral" => Some(ItemKind::HornCoral),
            "minecraft:dead_brain_coral" => Some(ItemKind::DeadBrainCoral),
            "minecraft:dead_bubble_coral" => Some(ItemKind::DeadBubbleCoral),
            "minecraft:dead_fire_coral" => Some(ItemKind::DeadFireCoral),
            "minecraft:dead_horn_coral" => Some(ItemKind::DeadHornCoral),
            "minecraft:dead_tube_coral" => Some(ItemKind::DeadTubeCoral),
            "minecraft:tube_coral_fan" => Some(ItemKind::TubeCoralFan),
            "minecraft:brain_coral_fan" => Some(ItemKind::BrainCoralFan),
            "minecraft:bubble_coral_fan" => Some(ItemKind::BubbleCoralFan),
            "minecraft:fire_coral_fan" => Some(ItemKind::FireCoralFan),
            "minecraft:horn_coral_fan" => Some(ItemKind::HornCoralFan),
            "minecraft:dead_tube_coral_fan" => Some(ItemKind::DeadTubeCoralFan),
            "minecraft:dead_brain_coral_fan" => Some(ItemKind::DeadBrainCoralFan),
            "minecraft:dead_bubble_coral_fan" => Some(ItemKind::DeadBubbleCoralFan),
            "minecraft:dead_fire_coral_fan" => Some(ItemKind::DeadFireCoralFan),
            "minecraft:dead_horn_coral_fan" => Some(ItemKind::DeadHornCoralFan),
            "minecraft:blue_ice" => Some(ItemKind::BlueIce),
            "minecraft:conduit" => Some(ItemKind::Conduit),
            "minecraft:polished_granite_stairs" => Some(ItemKind::PolishedGraniteStairs),
            "minecraft:smooth_red_sandstone_stairs" => Some(ItemKind::SmoothRedSandstoneStairs),
            "minecraft:mossy_stone_brick_stairs" => Some(ItemKind::MossyStoneBrickStairs),
            "minecraft:polished_diorite_stairs" => Some(ItemKind::PolishedDioriteStairs),
            "minecraft:mossy_cobblestone_stairs" => Some(ItemKind::MossyCobblestoneStairs),
            "minecraft:end_stone_brick_stairs" => Some(ItemKind::EndStoneBrickStairs),
            "minecraft:stone_stairs" => Some(ItemKind::StoneStairs),
            "minecraft:smooth_sandstone_stairs" => Some(ItemKind::SmoothSandstoneStairs),
            "minecraft:smooth_quartz_stairs" => Some(ItemKind::SmoothQuartzStairs),
            "minecraft:granite_stairs" => Some(ItemKind::GraniteStairs),
            "minecraft:andesite_stairs" => Some(ItemKind::AndesiteStairs),
            "minecraft:red_nether_brick_stairs" => Some(ItemKind::RedNetherBrickStairs),
            "minecraft:polished_andesite_stairs" => Some(ItemKind::PolishedAndesiteStairs),
            "minecraft:diorite_stairs" => Some(ItemKind::DioriteStairs),
            "minecraft:cobbled_deepslate_stairs" => Some(ItemKind::CobbledDeepslateStairs),
            "minecraft:polished_deepslate_stairs" => Some(ItemKind::PolishedDeepslateStairs),
            "minecraft:deepslate_brick_stairs" => Some(ItemKind::DeepslateBrickStairs),
            "minecraft:deepslate_tile_stairs" => Some(ItemKind::DeepslateTileStairs),
            "minecraft:polished_granite_slab" => Some(ItemKind::PolishedGraniteSlab),
            "minecraft:smooth_red_sandstone_slab" => Some(ItemKind::SmoothRedSandstoneSlab),
            "minecraft:mossy_stone_brick_slab" => Some(ItemKind::MossyStoneBrickSlab),
            "minecraft:polished_diorite_slab" => Some(ItemKind::PolishedDioriteSlab),
            "minecraft:mossy_cobblestone_slab" => Some(ItemKind::MossyCobblestoneSlab),
            "minecraft:end_stone_brick_slab" => Some(ItemKind::EndStoneBrickSlab),
            "minecraft:smooth_sandstone_slab" => Some(ItemKind::SmoothSandstoneSlab),
            "minecraft:smooth_quartz_slab" => Some(ItemKind::SmoothQuartzSlab),
            "minecraft:granite_slab" => Some(ItemKind::GraniteSlab),
            "minecraft:andesite_slab" => Some(ItemKind::AndesiteSlab),
            "minecraft:red_nether_brick_slab" => Some(ItemKind::RedNetherBrickSlab),
            "minecraft:polished_andesite_slab" => Some(ItemKind::PolishedAndesiteSlab),
            "minecraft:diorite_slab" => Some(ItemKind::DioriteSlab),
            "minecraft:cobbled_deepslate_slab" => Some(ItemKind::CobbledDeepslateSlab),
            "minecraft:polished_deepslate_slab" => Some(ItemKind::PolishedDeepslateSlab),
            "minecraft:deepslate_brick_slab" => Some(ItemKind::DeepslateBrickSlab),
            "minecraft:deepslate_tile_slab" => Some(ItemKind::DeepslateTileSlab),
            "minecraft:scaffolding" => Some(ItemKind::Scaffolding),
            "minecraft:redstone" => Some(ItemKind::Redstone),
            "minecraft:redstone_torch" => Some(ItemKind::RedstoneTorch),
            "minecraft:redstone_block" => Some(ItemKind::RedstoneBlock),
            "minecraft:repeater" => Some(ItemKind::Repeater),
            "minecraft:comparator" => Some(ItemKind::Comparator),
            "minecraft:piston" => Some(ItemKind::Piston),
            "minecraft:sticky_piston" => Some(ItemKind::StickyPiston),
            "minecraft:slime_block" => Some(ItemKind::SlimeBlock),
            "minecraft:honey_block" => Some(ItemKind::HoneyBlock),
            "minecraft:observer" => Some(ItemKind::Observer),
            "minecraft:hopper" => Some(ItemKind::Hopper),
            "minecraft:dispenser" => Some(ItemKind::Dispenser),
            "minecraft:dropper" => Some(ItemKind::Dropper),
            "minecraft:lectern" => Some(ItemKind::Lectern),
            "minecraft:target" => Some(ItemKind::Target),
            "minecraft:lever" => Some(ItemKind::Lever),
            "minecraft:lightning_rod" => Some(ItemKind::LightningRod),
            "minecraft:daylight_detector" => Some(ItemKind::DaylightDetector),
            "minecraft:sculk_sensor" => Some(ItemKind::SculkSensor),
            "minecraft:tripwire_hook" => Some(ItemKind::TripwireHook),
            "minecraft:trapped_chest" => Some(ItemKind::TrappedChest),
            "minecraft:tnt" => Some(ItemKind::Tnt),
            "minecraft:redstone_lamp" => Some(ItemKind::RedstoneLamp),
            "minecraft:note_block" => Some(ItemKind::NoteBlock),
            "minecraft:stone_button" => Some(ItemKind::StoneButton),
            "minecraft:polished_blackstone_button" => Some(ItemKind::PolishedBlackstoneButton),
            "minecraft:oak_button" => Some(ItemKind::OakButton),
            "minecraft:spruce_button" => Some(ItemKind::SpruceButton),
            "minecraft:birch_button" => Some(ItemKind::BirchButton),
            "minecraft:jungle_button" => Some(ItemKind::JungleButton),
            "minecraft:acacia_button" => Some(ItemKind::AcaciaButton),
            "minecraft:dark_oak_button" => Some(ItemKind::DarkOakButton),
            "minecraft:crimson_button" => Some(ItemKind::CrimsonButton),
            "minecraft:warped_button" => Some(ItemKind::WarpedButton),
            "minecraft:stone_pressure_plate" => Some(ItemKind::StonePressurePlate),
            "minecraft:polished_blackstone_pressure_plate" => {
                Some(ItemKind::PolishedBlackstonePressurePlate)
            }
            "minecraft:light_weighted_pressure_plate" => Some(ItemKind::LightWeightedPressurePlate),
            "minecraft:heavy_weighted_pressure_plate" => Some(ItemKind::HeavyWeightedPressurePlate),
            "minecraft:oak_pressure_plate" => Some(ItemKind::OakPressurePlate),
            "minecraft:spruce_pressure_plate" => Some(ItemKind::SprucePressurePlate),
            "minecraft:birch_pressure_plate" => Some(ItemKind::BirchPressurePlate),
            "minecraft:jungle_pressure_plate" => Some(ItemKind::JunglePressurePlate),
            "minecraft:acacia_pressure_plate" => Some(ItemKind::AcaciaPressurePlate),
            "minecraft:dark_oak_pressure_plate" => Some(ItemKind::DarkOakPressurePlate),
            "minecraft:crimson_pressure_plate" => Some(ItemKind::CrimsonPressurePlate),
            "minecraft:warped_pressure_plate" => Some(ItemKind::WarpedPressurePlate),
            "minecraft:iron_door" => Some(ItemKind::IronDoor),
            "minecraft:oak_door" => Some(ItemKind::OakDoor),
            "minecraft:spruce_door" => Some(ItemKind::SpruceDoor),
            "minecraft:birch_door" => Some(ItemKind::BirchDoor),
            "minecraft:jungle_door" => Some(ItemKind::JungleDoor),
            "minecraft:acacia_door" => Some(ItemKind::AcaciaDoor),
            "minecraft:dark_oak_door" => Some(ItemKind::DarkOakDoor),
            "minecraft:crimson_door" => Some(ItemKind::CrimsonDoor),
            "minecraft:warped_door" => Some(ItemKind::WarpedDoor),
            "minecraft:iron_trapdoor" => Some(ItemKind::IronTrapdoor),
            "minecraft:oak_trapdoor" => Some(ItemKind::OakTrapdoor),
            "minecraft:spruce_trapdoor" => Some(ItemKind::SpruceTrapdoor),
            "minecraft:birch_trapdoor" => Some(ItemKind::BirchTrapdoor),
            "minecraft:jungle_trapdoor" => Some(ItemKind::JungleTrapdoor),
            "minecraft:acacia_trapdoor" => Some(ItemKind::AcaciaTrapdoor),
            "minecraft:dark_oak_trapdoor" => Some(ItemKind::DarkOakTrapdoor),
            "minecraft:crimson_trapdoor" => Some(ItemKind::CrimsonTrapdoor),
            "minecraft:warped_trapdoor" => Some(ItemKind::WarpedTrapdoor),
            "minecraft:oak_fence_gate" => Some(ItemKind::OakFenceGate),
            "minecraft:spruce_fence_gate" => Some(ItemKind::SpruceFenceGate),
            "minecraft:birch_fence_gate" => Some(ItemKind::BirchFenceGate),
            "minecraft:jungle_fence_gate" => Some(ItemKind::JungleFenceGate),
            "minecraft:acacia_fence_gate" => Some(ItemKind::AcaciaFenceGate),
            "minecraft:dark_oak_fence_gate" => Some(ItemKind::DarkOakFenceGate),
            "minecraft:crimson_fence_gate" => Some(ItemKind::CrimsonFenceGate),
            "minecraft:warped_fence_gate" => Some(ItemKind::WarpedFenceGate),
            "minecraft:powered_rail" => Some(ItemKind::PoweredRail),
            "minecraft:detector_rail" => Some(ItemKind::DetectorRail),
            "minecraft:rail" => Some(ItemKind::Rail),
            "minecraft:activator_rail" => Some(ItemKind::ActivatorRail),
            "minecraft:saddle" => Some(ItemKind::Saddle),
            "minecraft:minecart" => Some(ItemKind::Minecart),
            "minecraft:chest_minecart" => Some(ItemKind::ChestMinecart),
            "minecraft:furnace_minecart" => Some(ItemKind::FurnaceMinecart),
            "minecraft:tnt_minecart" => Some(ItemKind::TntMinecart),
            "minecraft:hopper_minecart" => Some(ItemKind::HopperMinecart),
            "minecraft:carrot_on_a_stick" => Some(ItemKind::CarrotOnAStick),
            "minecraft:warped_fungus_on_a_stick" => Some(ItemKind::WarpedFungusOnAStick),
            "minecraft:elytra" => Some(ItemKind::Elytra),
            "minecraft:oak_boat" => Some(ItemKind::OakBoat),
            "minecraft:spruce_boat" => Some(ItemKind::SpruceBoat),
            "minecraft:birch_boat" => Some(ItemKind::BirchBoat),
            "minecraft:jungle_boat" => Some(ItemKind::JungleBoat),
            "minecraft:acacia_boat" => Some(ItemKind::AcaciaBoat),
            "minecraft:dark_oak_boat" => Some(ItemKind::DarkOakBoat),
            "minecraft:structure_block" => Some(ItemKind::StructureBlock),
            "minecraft:jigsaw" => Some(ItemKind::Jigsaw),
            "minecraft:turtle_helmet" => Some(ItemKind::TurtleHelmet),
            "minecraft:scute" => Some(ItemKind::Scute),
            "minecraft:flint_and_steel" => Some(ItemKind::FlintAndSteel),
            "minecraft:apple" => Some(ItemKind::Apple),
            "minecraft:bow" => Some(ItemKind::Bow),
            "minecraft:arrow" => Some(ItemKind::Arrow),
            "minecraft:coal" => Some(ItemKind::Coal),
            "minecraft:charcoal" => Some(ItemKind::Charcoal),
            "minecraft:diamond" => Some(ItemKind::Diamond),
            "minecraft:emerald" => Some(ItemKind::Emerald),
            "minecraft:lapis_lazuli" => Some(ItemKind::LapisLazuli),
            "minecraft:quartz" => Some(ItemKind::Quartz),
            "minecraft:amethyst_shard" => Some(ItemKind::AmethystShard),
            "minecraft:raw_iron" => Some(ItemKind::RawIron),
            "minecraft:iron_ingot" => Some(ItemKind::IronIngot),
            "minecraft:raw_copper" => Some(ItemKind::RawCopper),
            "minecraft:copper_ingot" => Some(ItemKind::CopperIngot),
            "minecraft:raw_gold" => Some(ItemKind::RawGold),
            "minecraft:gold_ingot" => Some(ItemKind::GoldIngot),
            "minecraft:netherite_ingot" => Some(ItemKind::NetheriteIngot),
            "minecraft:netherite_scrap" => Some(ItemKind::NetheriteScrap),
            "minecraft:wooden_sword" => Some(ItemKind::WoodenSword),
            "minecraft:wooden_shovel" => Some(ItemKind::WoodenShovel),
            "minecraft:wooden_pickaxe" => Some(ItemKind::WoodenPickaxe),
            "minecraft:wooden_axe" => Some(ItemKind::WoodenAxe),
            "minecraft:wooden_hoe" => Some(ItemKind::WoodenHoe),
            "minecraft:stone_sword" => Some(ItemKind::StoneSword),
            "minecraft:stone_shovel" => Some(ItemKind::StoneShovel),
            "minecraft:stone_pickaxe" => Some(ItemKind::StonePickaxe),
            "minecraft:stone_axe" => Some(ItemKind::StoneAxe),
            "minecraft:stone_hoe" => Some(ItemKind::StoneHoe),
            "minecraft:golden_sword" => Some(ItemKind::GoldenSword),
            "minecraft:golden_shovel" => Some(ItemKind::GoldenShovel),
            "minecraft:golden_pickaxe" => Some(ItemKind::GoldenPickaxe),
            "minecraft:golden_axe" => Some(ItemKind::GoldenAxe),
            "minecraft:golden_hoe" => Some(ItemKind::GoldenHoe),
            "minecraft:iron_sword" => Some(ItemKind::IronSword),
            "minecraft:iron_shovel" => Some(ItemKind::IronShovel),
            "minecraft:iron_pickaxe" => Some(ItemKind::IronPickaxe),
            "minecraft:iron_axe" => Some(ItemKind::IronAxe),
            "minecraft:iron_hoe" => Some(ItemKind::IronHoe),
            "minecraft:diamond_sword" => Some(ItemKind::DiamondSword),
            "minecraft:diamond_shovel" => Some(ItemKind::DiamondShovel),
            "minecraft:diamond_pickaxe" => Some(ItemKind::DiamondPickaxe),
            "minecraft:diamond_axe" => Some(ItemKind::DiamondAxe),
            "minecraft:diamond_hoe" => Some(ItemKind::DiamondHoe),
            "minecraft:netherite_sword" => Some(ItemKind::NetheriteSword),
            "minecraft:netherite_shovel" => Some(ItemKind::NetheriteShovel),
            "minecraft:netherite_pickaxe" => Some(ItemKind::NetheritePickaxe),
            "minecraft:netherite_axe" => Some(ItemKind::NetheriteAxe),
            "minecraft:netherite_hoe" => Some(ItemKind::NetheriteHoe),
            "minecraft:stick" => Some(ItemKind::Stick),
            "minecraft:bowl" => Some(ItemKind::Bowl),
            "minecraft:mushroom_stew" => Some(ItemKind::MushroomStew),
            "minecraft:string" => Some(ItemKind::String),
            "minecraft:feather" => Some(ItemKind::Feather),
            "minecraft:gunpowder" => Some(ItemKind::Gunpowder),
            "minecraft:wheat_seeds" => Some(ItemKind::WheatSeeds),
            "minecraft:wheat" => Some(ItemKind::Wheat),
            "minecraft:bread" => Some(ItemKind::Bread),
            "minecraft:leather_helmet" => Some(ItemKind::LeatherHelmet),
            "minecraft:leather_chestplate" => Some(ItemKind::LeatherChestplate),
            "minecraft:leather_leggings" => Some(ItemKind::LeatherLeggings),
            "minecraft:leather_boots" => Some(ItemKind::LeatherBoots),
            "minecraft:chainmail_helmet" => Some(ItemKind::ChainmailHelmet),
            "minecraft:chainmail_chestplate" => Some(ItemKind::ChainmailChestplate),
            "minecraft:chainmail_leggings" => Some(ItemKind::ChainmailLeggings),
            "minecraft:chainmail_boots" => Some(ItemKind::ChainmailBoots),
            "minecraft:iron_helmet" => Some(ItemKind::IronHelmet),
            "minecraft:iron_chestplate" => Some(ItemKind::IronChestplate),
            "minecraft:iron_leggings" => Some(ItemKind::IronLeggings),
            "minecraft:iron_boots" => Some(ItemKind::IronBoots),
            "minecraft:diamond_helmet" => Some(ItemKind::DiamondHelmet),
            "minecraft:diamond_chestplate" => Some(ItemKind::DiamondChestplate),
            "minecraft:diamond_leggings" => Some(ItemKind::DiamondLeggings),
            "minecraft:diamond_boots" => Some(ItemKind::DiamondBoots),
            "minecraft:golden_helmet" => Some(ItemKind::GoldenHelmet),
            "minecraft:golden_chestplate" => Some(ItemKind::GoldenChestplate),
            "minecraft:golden_leggings" => Some(ItemKind::GoldenLeggings),
            "minecraft:golden_boots" => Some(ItemKind::GoldenBoots),
            "minecraft:netherite_helmet" => Some(ItemKind::NetheriteHelmet),
            "minecraft:netherite_chestplate" => Some(ItemKind::NetheriteChestplate),
            "minecraft:netherite_leggings" => Some(ItemKind::NetheriteLeggings),
            "minecraft:netherite_boots" => Some(ItemKind::NetheriteBoots),
            "minecraft:flint" => Some(ItemKind::Flint),
            "minecraft:porkchop" => Some(ItemKind::Porkchop),
            "minecraft:cooked_porkchop" => Some(ItemKind::CookedPorkchop),
            "minecraft:painting" => Some(ItemKind::Painting),
            "minecraft:golden_apple" => Some(ItemKind::GoldenApple),
            "minecraft:enchanted_golden_apple" => Some(ItemKind::EnchantedGoldenApple),
            "minecraft:oak_sign" => Some(ItemKind::OakSign),
            "minecraft:spruce_sign" => Some(ItemKind::SpruceSign),
            "minecraft:birch_sign" => Some(ItemKind::BirchSign),
            "minecraft:jungle_sign" => Some(ItemKind::JungleSign),
            "minecraft:acacia_sign" => Some(ItemKind::AcaciaSign),
            "minecraft:dark_oak_sign" => Some(ItemKind::DarkOakSign),
            "minecraft:crimson_sign" => Some(ItemKind::CrimsonSign),
            "minecraft:warped_sign" => Some(ItemKind::WarpedSign),
            "minecraft:bucket" => Some(ItemKind::Bucket),
            "minecraft:water_bucket" => Some(ItemKind::WaterBucket),
            "minecraft:lava_bucket" => Some(ItemKind::LavaBucket),
            "minecraft:powder_snow_bucket" => Some(ItemKind::PowderSnowBucket),
            "minecraft:snowball" => Some(ItemKind::Snowball),
            "minecraft:leather" => Some(ItemKind::Leather),
            "minecraft:milk_bucket" => Some(ItemKind::MilkBucket),
            "minecraft:pufferfish_bucket" => Some(ItemKind::PufferfishBucket),
            "minecraft:salmon_bucket" => Some(ItemKind::SalmonBucket),
            "minecraft:cod_bucket" => Some(ItemKind::CodBucket),
            "minecraft:tropical_fish_bucket" => Some(ItemKind::TropicalFishBucket),
            "minecraft:axolotl_bucket" => Some(ItemKind::AxolotlBucket),
            "minecraft:brick" => Some(ItemKind::Brick),
            "minecraft:clay_ball" => Some(ItemKind::ClayBall),
            "minecraft:dried_kelp_block" => Some(ItemKind::DriedKelpBlock),
            "minecraft:paper" => Some(ItemKind::Paper),
            "minecraft:book" => Some(ItemKind::Book),
            "minecraft:slime_ball" => Some(ItemKind::SlimeBall),
            "minecraft:egg" => Some(ItemKind::Egg),
            "minecraft:compass" => Some(ItemKind::Compass),
            "minecraft:bundle" => Some(ItemKind::Bundle),
            "minecraft:fishing_rod" => Some(ItemKind::FishingRod),
            "minecraft:clock" => Some(ItemKind::Clock),
            "minecraft:spyglass" => Some(ItemKind::Spyglass),
            "minecraft:glowstone_dust" => Some(ItemKind::GlowstoneDust),
            "minecraft:cod" => Some(ItemKind::Cod),
            "minecraft:salmon" => Some(ItemKind::Salmon),
            "minecraft:tropical_fish" => Some(ItemKind::TropicalFish),
            "minecraft:pufferfish" => Some(ItemKind::Pufferfish),
            "minecraft:cooked_cod" => Some(ItemKind::CookedCod),
            "minecraft:cooked_salmon" => Some(ItemKind::CookedSalmon),
            "minecraft:ink_sac" => Some(ItemKind::InkSac),
            "minecraft:glow_ink_sac" => Some(ItemKind::GlowInkSac),
            "minecraft:cocoa_beans" => Some(ItemKind::CocoaBeans),
            "minecraft:white_dye" => Some(ItemKind::WhiteDye),
            "minecraft:orange_dye" => Some(ItemKind::OrangeDye),
            "minecraft:magenta_dye" => Some(ItemKind::MagentaDye),
            "minecraft:light_blue_dye" => Some(ItemKind::LightBlueDye),
            "minecraft:yellow_dye" => Some(ItemKind::YellowDye),
            "minecraft:lime_dye" => Some(ItemKind::LimeDye),
            "minecraft:pink_dye" => Some(ItemKind::PinkDye),
            "minecraft:gray_dye" => Some(ItemKind::GrayDye),
            "minecraft:light_gray_dye" => Some(ItemKind::LightGrayDye),
            "minecraft:cyan_dye" => Some(ItemKind::CyanDye),
            "minecraft:purple_dye" => Some(ItemKind::PurpleDye),
            "minecraft:blue_dye" => Some(ItemKind::BlueDye),
            "minecraft:brown_dye" => Some(ItemKind::BrownDye),
            "minecraft:green_dye" => Some(ItemKind::GreenDye),
            "minecraft:red_dye" => Some(ItemKind::RedDye),
            "minecraft:black_dye" => Some(ItemKind::BlackDye),
            "minecraft:bone_meal" => Some(ItemKind::BoneMeal),
            "minecraft:bone" => Some(ItemKind::Bone),
            "minecraft:sugar" => Some(ItemKind::Sugar),
            "minecraft:cake" => Some(ItemKind::Cake),
            "minecraft:white_bed" => Some(ItemKind::WhiteBed),
            "minecraft:orange_bed" => Some(ItemKind::OrangeBed),
            "minecraft:magenta_bed" => Some(ItemKind::MagentaBed),
            "minecraft:light_blue_bed" => Some(ItemKind::LightBlueBed),
            "minecraft:yellow_bed" => Some(ItemKind::YellowBed),
            "minecraft:lime_bed" => Some(ItemKind::LimeBed),
            "minecraft:pink_bed" => Some(ItemKind::PinkBed),
            "minecraft:gray_bed" => Some(ItemKind::GrayBed),
            "minecraft:light_gray_bed" => Some(ItemKind::LightGrayBed),
            "minecraft:cyan_bed" => Some(ItemKind::CyanBed),
            "minecraft:purple_bed" => Some(ItemKind::PurpleBed),
            "minecraft:blue_bed" => Some(ItemKind::BlueBed),
            "minecraft:brown_bed" => Some(ItemKind::BrownBed),
            "minecraft:green_bed" => Some(ItemKind::GreenBed),
            "minecraft:red_bed" => Some(ItemKind::RedBed),
            "minecraft:black_bed" => Some(ItemKind::BlackBed),
            "minecraft:cookie" => Some(ItemKind::Cookie),
            "minecraft:filled_map" => Some(ItemKind::FilledMap),
            "minecraft:shears" => Some(ItemKind::Shears),
            "minecraft:melon_slice" => Some(ItemKind::MelonSlice),
            "minecraft:dried_kelp" => Some(ItemKind::DriedKelp),
            "minecraft:pumpkin_seeds" => Some(ItemKind::PumpkinSeeds),
            "minecraft:melon_seeds" => Some(ItemKind::MelonSeeds),
            "minecraft:beef" => Some(ItemKind::Beef),
            "minecraft:cooked_beef" => Some(ItemKind::CookedBeef),
            "minecraft:chicken" => Some(ItemKind::Chicken),
            "minecraft:cooked_chicken" => Some(ItemKind::CookedChicken),
            "minecraft:rotten_flesh" => Some(ItemKind::RottenFlesh),
            "minecraft:ender_pearl" => Some(ItemKind::EnderPearl),
            "minecraft:blaze_rod" => Some(ItemKind::BlazeRod),
            "minecraft:ghast_tear" => Some(ItemKind::GhastTear),
            "minecraft:gold_nugget" => Some(ItemKind::GoldNugget),
            "minecraft:nether_wart" => Some(ItemKind::NetherWart),
            "minecraft:potion" => Some(ItemKind::Potion),
            "minecraft:glass_bottle" => Some(ItemKind::GlassBottle),
            "minecraft:spider_eye" => Some(ItemKind::SpiderEye),
            "minecraft:fermented_spider_eye" => Some(ItemKind::FermentedSpiderEye),
            "minecraft:blaze_powder" => Some(ItemKind::BlazePowder),
            "minecraft:magma_cream" => Some(ItemKind::MagmaCream),
            "minecraft:brewing_stand" => Some(ItemKind::BrewingStand),
            "minecraft:cauldron" => Some(ItemKind::Cauldron),
            "minecraft:ender_eye" => Some(ItemKind::EnderEye),
            "minecraft:glistering_melon_slice" => Some(ItemKind::GlisteringMelonSlice),
            "minecraft:axolotl_spawn_egg" => Some(ItemKind::AxolotlSpawnEgg),
            "minecraft:bat_spawn_egg" => Some(ItemKind::BatSpawnEgg),
            "minecraft:bee_spawn_egg" => Some(ItemKind::BeeSpawnEgg),
            "minecraft:blaze_spawn_egg" => Some(ItemKind::BlazeSpawnEgg),
            "minecraft:cat_spawn_egg" => Some(ItemKind::CatSpawnEgg),
            "minecraft:cave_spider_spawn_egg" => Some(ItemKind::CaveSpiderSpawnEgg),
            "minecraft:chicken_spawn_egg" => Some(ItemKind::ChickenSpawnEgg),
            "minecraft:cod_spawn_egg" => Some(ItemKind::CodSpawnEgg),
            "minecraft:cow_spawn_egg" => Some(ItemKind::CowSpawnEgg),
            "minecraft:creeper_spawn_egg" => Some(ItemKind::CreeperSpawnEgg),
            "minecraft:dolphin_spawn_egg" => Some(ItemKind::DolphinSpawnEgg),
            "minecraft:donkey_spawn_egg" => Some(ItemKind::DonkeySpawnEgg),
            "minecraft:drowned_spawn_egg" => Some(ItemKind::DrownedSpawnEgg),
            "minecraft:elder_guardian_spawn_egg" => Some(ItemKind::ElderGuardianSpawnEgg),
            "minecraft:enderman_spawn_egg" => Some(ItemKind::EndermanSpawnEgg),
            "minecraft:endermite_spawn_egg" => Some(ItemKind::EndermiteSpawnEgg),
            "minecraft:evoker_spawn_egg" => Some(ItemKind::EvokerSpawnEgg),
            "minecraft:fox_spawn_egg" => Some(ItemKind::FoxSpawnEgg),
            "minecraft:ghast_spawn_egg" => Some(ItemKind::GhastSpawnEgg),
            "minecraft:glow_squid_spawn_egg" => Some(ItemKind::GlowSquidSpawnEgg),
            "minecraft:goat_spawn_egg" => Some(ItemKind::GoatSpawnEgg),
            "minecraft:guardian_spawn_egg" => Some(ItemKind::GuardianSpawnEgg),
            "minecraft:hoglin_spawn_egg" => Some(ItemKind::HoglinSpawnEgg),
            "minecraft:horse_spawn_egg" => Some(ItemKind::HorseSpawnEgg),
            "minecraft:husk_spawn_egg" => Some(ItemKind::HuskSpawnEgg),
            "minecraft:llama_spawn_egg" => Some(ItemKind::LlamaSpawnEgg),
            "minecraft:magma_cube_spawn_egg" => Some(ItemKind::MagmaCubeSpawnEgg),
            "minecraft:mooshroom_spawn_egg" => Some(ItemKind::MooshroomSpawnEgg),
            "minecraft:mule_spawn_egg" => Some(ItemKind::MuleSpawnEgg),
            "minecraft:ocelot_spawn_egg" => Some(ItemKind::OcelotSpawnEgg),
            "minecraft:panda_spawn_egg" => Some(ItemKind::PandaSpawnEgg),
            "minecraft:parrot_spawn_egg" => Some(ItemKind::ParrotSpawnEgg),
            "minecraft:phantom_spawn_egg" => Some(ItemKind::PhantomSpawnEgg),
            "minecraft:pig_spawn_egg" => Some(ItemKind::PigSpawnEgg),
            "minecraft:piglin_spawn_egg" => Some(ItemKind::PiglinSpawnEgg),
            "minecraft:piglin_brute_spawn_egg" => Some(ItemKind::PiglinBruteSpawnEgg),
            "minecraft:pillager_spawn_egg" => Some(ItemKind::PillagerSpawnEgg),
            "minecraft:polar_bear_spawn_egg" => Some(ItemKind::PolarBearSpawnEgg),
            "minecraft:pufferfish_spawn_egg" => Some(ItemKind::PufferfishSpawnEgg),
            "minecraft:rabbit_spawn_egg" => Some(ItemKind::RabbitSpawnEgg),
            "minecraft:ravager_spawn_egg" => Some(ItemKind::RavagerSpawnEgg),
            "minecraft:salmon_spawn_egg" => Some(ItemKind::SalmonSpawnEgg),
            "minecraft:sheep_spawn_egg" => Some(ItemKind::SheepSpawnEgg),
            "minecraft:shulker_spawn_egg" => Some(ItemKind::ShulkerSpawnEgg),
            "minecraft:silverfish_spawn_egg" => Some(ItemKind::SilverfishSpawnEgg),
            "minecraft:skeleton_spawn_egg" => Some(ItemKind::SkeletonSpawnEgg),
            "minecraft:skeleton_horse_spawn_egg" => Some(ItemKind::SkeletonHorseSpawnEgg),
            "minecraft:slime_spawn_egg" => Some(ItemKind::SlimeSpawnEgg),
            "minecraft:spider_spawn_egg" => Some(ItemKind::SpiderSpawnEgg),
            "minecraft:squid_spawn_egg" => Some(ItemKind::SquidSpawnEgg),
            "minecraft:stray_spawn_egg" => Some(ItemKind::StraySpawnEgg),
            "minecraft:strider_spawn_egg" => Some(ItemKind::StriderSpawnEgg),
            "minecraft:trader_llama_spawn_egg" => Some(ItemKind::TraderLlamaSpawnEgg),
            "minecraft:tropical_fish_spawn_egg" => Some(ItemKind::TropicalFishSpawnEgg),
            "minecraft:turtle_spawn_egg" => Some(ItemKind::TurtleSpawnEgg),
            "minecraft:vex_spawn_egg" => Some(ItemKind::VexSpawnEgg),
            "minecraft:villager_spawn_egg" => Some(ItemKind::VillagerSpawnEgg),
            "minecraft:vindicator_spawn_egg" => Some(ItemKind::VindicatorSpawnEgg),
            "minecraft:wandering_trader_spawn_egg" => Some(ItemKind::WanderingTraderSpawnEgg),
            "minecraft:witch_spawn_egg" => Some(ItemKind::WitchSpawnEgg),
            "minecraft:wither_skeleton_spawn_egg" => Some(ItemKind::WitherSkeletonSpawnEgg),
            "minecraft:wolf_spawn_egg" => Some(ItemKind::WolfSpawnEgg),
            "minecraft:zoglin_spawn_egg" => Some(ItemKind::ZoglinSpawnEgg),
            "minecraft:zombie_spawn_egg" => Some(ItemKind::ZombieSpawnEgg),
            "minecraft:zombie_horse_spawn_egg" => Some(ItemKind::ZombieHorseSpawnEgg),
            "minecraft:zombie_villager_spawn_egg" => Some(ItemKind::ZombieVillagerSpawnEgg),
            "minecraft:zombified_piglin_spawn_egg" => Some(ItemKind::ZombifiedPiglinSpawnEgg),
            "minecraft:experience_bottle" => Some(ItemKind::ExperienceBottle),
            "minecraft:fire_charge" => Some(ItemKind::FireCharge),
            "minecraft:writable_book" => Some(ItemKind::WritableBook),
            "minecraft:written_book" => Some(ItemKind::WrittenBook),
            "minecraft:item_frame" => Some(ItemKind::ItemFrame),
            "minecraft:glow_item_frame" => Some(ItemKind::GlowItemFrame),
            "minecraft:flower_pot" => Some(ItemKind::FlowerPot),
            "minecraft:carrot" => Some(ItemKind::Carrot),
            "minecraft:potato" => Some(ItemKind::Potato),
            "minecraft:baked_potato" => Some(ItemKind::BakedPotato),
            "minecraft:poisonous_potato" => Some(ItemKind::PoisonousPotato),
            "minecraft:map" => Some(ItemKind::Map),
            "minecraft:golden_carrot" => Some(ItemKind::GoldenCarrot),
            "minecraft:skeleton_skull" => Some(ItemKind::SkeletonSkull),
            "minecraft:wither_skeleton_skull" => Some(ItemKind::WitherSkeletonSkull),
            "minecraft:player_head" => Some(ItemKind::PlayerHead),
            "minecraft:zombie_head" => Some(ItemKind::ZombieHead),
            "minecraft:creeper_head" => Some(ItemKind::CreeperHead),
            "minecraft:dragon_head" => Some(ItemKind::DragonHead),
            "minecraft:nether_star" => Some(ItemKind::NetherStar),
            "minecraft:pumpkin_pie" => Some(ItemKind::PumpkinPie),
            "minecraft:firework_rocket" => Some(ItemKind::FireworkRocket),
            "minecraft:firework_star" => Some(ItemKind::FireworkStar),
            "minecraft:enchanted_book" => Some(ItemKind::EnchantedBook),
            "minecraft:nether_brick" => Some(ItemKind::NetherBrick),
            "minecraft:prismarine_shard" => Some(ItemKind::PrismarineShard),
            "minecraft:prismarine_crystals" => Some(ItemKind::PrismarineCrystals),
            "minecraft:rabbit" => Some(ItemKind::Rabbit),
            "minecraft:cooked_rabbit" => Some(ItemKind::CookedRabbit),
            "minecraft:rabbit_stew" => Some(ItemKind::RabbitStew),
            "minecraft:rabbit_foot" => Some(ItemKind::RabbitFoot),
            "minecraft:rabbit_hide" => Some(ItemKind::RabbitHide),
            "minecraft:armor_stand" => Some(ItemKind::ArmorStand),
            "minecraft:iron_horse_armor" => Some(ItemKind::IronHorseArmor),
            "minecraft:golden_horse_armor" => Some(ItemKind::GoldenHorseArmor),
            "minecraft:diamond_horse_armor" => Some(ItemKind::DiamondHorseArmor),
            "minecraft:leather_horse_armor" => Some(ItemKind::LeatherHorseArmor),
            "minecraft:lead" => Some(ItemKind::Lead),
            "minecraft:name_tag" => Some(ItemKind::NameTag),
            "minecraft:command_block_minecart" => Some(ItemKind::CommandBlockMinecart),
            "minecraft:mutton" => Some(ItemKind::Mutton),
            "minecraft:cooked_mutton" => Some(ItemKind::CookedMutton),
            "minecraft:white_banner" => Some(ItemKind::WhiteBanner),
            "minecraft:orange_banner" => Some(ItemKind::OrangeBanner),
            "minecraft:magenta_banner" => Some(ItemKind::MagentaBanner),
            "minecraft:light_blue_banner" => Some(ItemKind::LightBlueBanner),
            "minecraft:yellow_banner" => Some(ItemKind::YellowBanner),
            "minecraft:lime_banner" => Some(ItemKind::LimeBanner),
            "minecraft:pink_banner" => Some(ItemKind::PinkBanner),
            "minecraft:gray_banner" => Some(ItemKind::GrayBanner),
            "minecraft:light_gray_banner" => Some(ItemKind::LightGrayBanner),
            "minecraft:cyan_banner" => Some(ItemKind::CyanBanner),
            "minecraft:purple_banner" => Some(ItemKind::PurpleBanner),
            "minecraft:blue_banner" => Some(ItemKind::BlueBanner),
            "minecraft:brown_banner" => Some(ItemKind::BrownBanner),
            "minecraft:green_banner" => Some(ItemKind::GreenBanner),
            "minecraft:red_banner" => Some(ItemKind::RedBanner),
            "minecraft:black_banner" => Some(ItemKind::BlackBanner),
            "minecraft:end_crystal" => Some(ItemKind::EndCrystal),
            "minecraft:chorus_fruit" => Some(ItemKind::ChorusFruit),
            "minecraft:popped_chorus_fruit" => Some(ItemKind::PoppedChorusFruit),
            "minecraft:beetroot" => Some(ItemKind::Beetroot),
            "minecraft:beetroot_seeds" => Some(ItemKind::BeetrootSeeds),
            "minecraft:beetroot_soup" => Some(ItemKind::BeetrootSoup),
            "minecraft:dragon_breath" => Some(ItemKind::DragonBreath),
            "minecraft:splash_potion" => Some(ItemKind::SplashPotion),
            "minecraft:spectral_arrow" => Some(ItemKind::SpectralArrow),
            "minecraft:tipped_arrow" => Some(ItemKind::TippedArrow),
            "minecraft:lingering_potion" => Some(ItemKind::LingeringPotion),
            "minecraft:shield" => Some(ItemKind::Shield),
            "minecraft:totem_of_undying" => Some(ItemKind::TotemOfUndying),
            "minecraft:shulker_shell" => Some(ItemKind::ShulkerShell),
            "minecraft:iron_nugget" => Some(ItemKind::IronNugget),
            "minecraft:knowledge_book" => Some(ItemKind::KnowledgeBook),
            "minecraft:debug_stick" => Some(ItemKind::DebugStick),
            "minecraft:music_disc_13" => Some(ItemKind::MusicDisc13),
            "minecraft:music_disc_cat" => Some(ItemKind::MusicDiscCat),
            "minecraft:music_disc_blocks" => Some(ItemKind::MusicDiscBlocks),
            "minecraft:music_disc_chirp" => Some(ItemKind::MusicDiscChirp),
            "minecraft:music_disc_far" => Some(ItemKind::MusicDiscFar),
            "minecraft:music_disc_mall" => Some(ItemKind::MusicDiscMall),
            "minecraft:music_disc_mellohi" => Some(ItemKind::MusicDiscMellohi),
            "minecraft:music_disc_stal" => Some(ItemKind::MusicDiscStal),
            "minecraft:music_disc_strad" => Some(ItemKind::MusicDiscStrad),
            "minecraft:music_disc_ward" => Some(ItemKind::MusicDiscWard),
            "minecraft:music_disc_11" => Some(ItemKind::MusicDisc11),
            "minecraft:music_disc_wait" => Some(ItemKind::MusicDiscWait),
            "minecraft:music_disc_otherside" => Some(ItemKind::MusicDiscOtherside),
            "minecraft:music_disc_pigstep" => Some(ItemKind::MusicDiscPigstep),
            "minecraft:trident" => Some(ItemKind::Trident),
            "minecraft:phantom_membrane" => Some(ItemKind::PhantomMembrane),
            "minecraft:nautilus_shell" => Some(ItemKind::NautilusShell),
            "minecraft:heart_of_the_sea" => Some(ItemKind::HeartOfTheSea),
            "minecraft:crossbow" => Some(ItemKind::Crossbow),
            "minecraft:suspicious_stew" => Some(ItemKind::SuspiciousStew),
            "minecraft:loom" => Some(ItemKind::Loom),
            "minecraft:flower_banner_pattern" => Some(ItemKind::FlowerBannerPattern),
            "minecraft:creeper_banner_pattern" => Some(ItemKind::CreeperBannerPattern),
            "minecraft:skull_banner_pattern" => Some(ItemKind::SkullBannerPattern),
            "minecraft:mojang_banner_pattern" => Some(ItemKind::MojangBannerPattern),
            "minecraft:globe_banner_pattern" => Some(ItemKind::GlobeBannerPattern),
            "minecraft:piglin_banner_pattern" => Some(ItemKind::PiglinBannerPattern),
            "minecraft:composter" => Some(ItemKind::Composter),
            "minecraft:barrel" => Some(ItemKind::Barrel),
            "minecraft:smoker" => Some(ItemKind::Smoker),
            "minecraft:blast_furnace" => Some(ItemKind::BlastFurnace),
            "minecraft:cartography_table" => Some(ItemKind::CartographyTable),
            "minecraft:fletching_table" => Some(ItemKind::FletchingTable),
            "minecraft:grindstone" => Some(ItemKind::Grindstone),
            "minecraft:smithing_table" => Some(ItemKind::SmithingTable),
            "minecraft:stonecutter" => Some(ItemKind::Stonecutter),
            "minecraft:bell" => Some(ItemKind::Bell),
            "minecraft:lantern" => Some(ItemKind::Lantern),
            "minecraft:soul_lantern" => Some(ItemKind::SoulLantern),
            "minecraft:sweet_berries" => Some(ItemKind::SweetBerries),
            "minecraft:glow_berries" => Some(ItemKind::GlowBerries),
            "minecraft:campfire" => Some(ItemKind::Campfire),
            "minecraft:soul_campfire" => Some(ItemKind::SoulCampfire),
            "minecraft:shroomlight" => Some(ItemKind::Shroomlight),
            "minecraft:honeycomb" => Some(ItemKind::Honeycomb),
            "minecraft:bee_nest" => Some(ItemKind::BeeNest),
            "minecraft:beehive" => Some(ItemKind::Beehive),
            "minecraft:honey_bottle" => Some(ItemKind::HoneyBottle),
            "minecraft:honeycomb_block" => Some(ItemKind::HoneycombBlock),
            "minecraft:lodestone" => Some(ItemKind::Lodestone),
            "minecraft:crying_obsidian" => Some(ItemKind::CryingObsidian),
            "minecraft:blackstone" => Some(ItemKind::Blackstone),
            "minecraft:blackstone_slab" => Some(ItemKind::BlackstoneSlab),
            "minecraft:blackstone_stairs" => Some(ItemKind::BlackstoneStairs),
            "minecraft:gilded_blackstone" => Some(ItemKind::GildedBlackstone),
            "minecraft:polished_blackstone" => Some(ItemKind::PolishedBlackstone),
            "minecraft:polished_blackstone_slab" => Some(ItemKind::PolishedBlackstoneSlab),
            "minecraft:polished_blackstone_stairs" => Some(ItemKind::PolishedBlackstoneStairs),
            "minecraft:chiseled_polished_blackstone" => Some(ItemKind::ChiseledPolishedBlackstone),
            "minecraft:polished_blackstone_bricks" => Some(ItemKind::PolishedBlackstoneBricks),
            "minecraft:polished_blackstone_brick_slab" => {
                Some(ItemKind::PolishedBlackstoneBrickSlab)
            }
            "minecraft:polished_blackstone_brick_stairs" => {
                Some(ItemKind::PolishedBlackstoneBrickStairs)
            }
            "minecraft:cracked_polished_blackstone_bricks" => {
                Some(ItemKind::CrackedPolishedBlackstoneBricks)
            }
            "minecraft:respawn_anchor" => Some(ItemKind::RespawnAnchor),
            "minecraft:candle" => Some(ItemKind::Candle),
            "minecraft:white_candle" => Some(ItemKind::WhiteCandle),
            "minecraft:orange_candle" => Some(ItemKind::OrangeCandle),
            "minecraft:magenta_candle" => Some(ItemKind::MagentaCandle),
            "minecraft:light_blue_candle" => Some(ItemKind::LightBlueCandle),
            "minecraft:yellow_candle" => Some(ItemKind::YellowCandle),
            "minecraft:lime_candle" => Some(ItemKind::LimeCandle),
            "minecraft:pink_candle" => Some(ItemKind::PinkCandle),
            "minecraft:gray_candle" => Some(ItemKind::GrayCandle),
            "minecraft:light_gray_candle" => Some(ItemKind::LightGrayCandle),
            "minecraft:cyan_candle" => Some(ItemKind::CyanCandle),
            "minecraft:purple_candle" => Some(ItemKind::PurpleCandle),
            "minecraft:blue_candle" => Some(ItemKind::BlueCandle),
            "minecraft:brown_candle" => Some(ItemKind::BrownCandle),
            "minecraft:green_candle" => Some(ItemKind::GreenCandle),
            "minecraft:red_candle" => Some(ItemKind::RedCandle),
            "minecraft:black_candle" => Some(ItemKind::BlackCandle),
            "minecraft:small_amethyst_bud" => Some(ItemKind::SmallAmethystBud),
            "minecraft:medium_amethyst_bud" => Some(ItemKind::MediumAmethystBud),
            "minecraft:large_amethyst_bud" => Some(ItemKind::LargeAmethystBud),
            "minecraft:amethyst_cluster" => Some(ItemKind::AmethystCluster),
            "minecraft:pointed_dripstone" => Some(ItemKind::PointedDripstone),
            _ => None,
        }
    }
}
impl ItemKind {
    #[doc = "Returns the `stack_size` property of this `ItemKind`."]
    #[inline]
    pub const fn stack_size(&self) -> u32 {
        match self {
            ItemKind::Stone => 64u32,
            ItemKind::Granite => 64u32,
            ItemKind::PolishedGranite => 64u32,
            ItemKind::Diorite => 64u32,
            ItemKind::PolishedDiorite => 64u32,
            ItemKind::Andesite => 64u32,
            ItemKind::PolishedAndesite => 64u32,
            ItemKind::Deepslate => 64u32,
            ItemKind::CobbledDeepslate => 64u32,
            ItemKind::PolishedDeepslate => 64u32,
            ItemKind::Calcite => 64u32,
            ItemKind::Tuff => 64u32,
            ItemKind::DripstoneBlock => 64u32,
            ItemKind::GrassBlock => 64u32,
            ItemKind::Dirt => 64u32,
            ItemKind::CoarseDirt => 64u32,
            ItemKind::Podzol => 64u32,
            ItemKind::RootedDirt => 64u32,
            ItemKind::CrimsonNylium => 64u32,
            ItemKind::WarpedNylium => 64u32,
            ItemKind::Cobblestone => 64u32,
            ItemKind::OakPlanks => 64u32,
            ItemKind::SprucePlanks => 64u32,
            ItemKind::BirchPlanks => 64u32,
            ItemKind::JunglePlanks => 64u32,
            ItemKind::AcaciaPlanks => 64u32,
            ItemKind::DarkOakPlanks => 64u32,
            ItemKind::CrimsonPlanks => 64u32,
            ItemKind::WarpedPlanks => 64u32,
            ItemKind::OakSapling => 64u32,
            ItemKind::SpruceSapling => 64u32,
            ItemKind::BirchSapling => 64u32,
            ItemKind::JungleSapling => 64u32,
            ItemKind::AcaciaSapling => 64u32,
            ItemKind::DarkOakSapling => 64u32,
            ItemKind::Bedrock => 64u32,
            ItemKind::Sand => 64u32,
            ItemKind::RedSand => 64u32,
            ItemKind::Gravel => 64u32,
            ItemKind::CoalOre => 64u32,
            ItemKind::DeepslateCoalOre => 64u32,
            ItemKind::IronOre => 64u32,
            ItemKind::DeepslateIronOre => 64u32,
            ItemKind::CopperOre => 64u32,
            ItemKind::DeepslateCopperOre => 64u32,
            ItemKind::GoldOre => 64u32,
            ItemKind::DeepslateGoldOre => 64u32,
            ItemKind::RedstoneOre => 64u32,
            ItemKind::DeepslateRedstoneOre => 64u32,
            ItemKind::EmeraldOre => 64u32,
            ItemKind::DeepslateEmeraldOre => 64u32,
            ItemKind::LapisOre => 64u32,
            ItemKind::DeepslateLapisOre => 64u32,
            ItemKind::DiamondOre => 64u32,
            ItemKind::DeepslateDiamondOre => 64u32,
            ItemKind::NetherGoldOre => 64u32,
            ItemKind::NetherQuartzOre => 64u32,
            ItemKind::AncientDebris => 64u32,
            ItemKind::CoalBlock => 64u32,
            ItemKind::RawIronBlock => 64u32,
            ItemKind::RawCopperBlock => 64u32,
            ItemKind::RawGoldBlock => 64u32,
            ItemKind::AmethystBlock => 64u32,
            ItemKind::BuddingAmethyst => 64u32,
            ItemKind::IronBlock => 64u32,
            ItemKind::CopperBlock => 64u32,
            ItemKind::GoldBlock => 64u32,
            ItemKind::DiamondBlock => 64u32,
            ItemKind::NetheriteBlock => 64u32,
            ItemKind::ExposedCopper => 64u32,
            ItemKind::WeatheredCopper => 64u32,
            ItemKind::OxidizedCopper => 64u32,
            ItemKind::CutCopper => 64u32,
            ItemKind::ExposedCutCopper => 64u32,
            ItemKind::WeatheredCutCopper => 64u32,
            ItemKind::OxidizedCutCopper => 64u32,
            ItemKind::CutCopperStairs => 64u32,
            ItemKind::ExposedCutCopperStairs => 64u32,
            ItemKind::WeatheredCutCopperStairs => 64u32,
            ItemKind::OxidizedCutCopperStairs => 64u32,
            ItemKind::CutCopperSlab => 64u32,
            ItemKind::ExposedCutCopperSlab => 64u32,
            ItemKind::WeatheredCutCopperSlab => 64u32,
            ItemKind::OxidizedCutCopperSlab => 64u32,
            ItemKind::WaxedCopperBlock => 64u32,
            ItemKind::WaxedExposedCopper => 64u32,
            ItemKind::WaxedWeatheredCopper => 64u32,
            ItemKind::WaxedOxidizedCopper => 64u32,
            ItemKind::WaxedCutCopper => 64u32,
            ItemKind::WaxedExposedCutCopper => 64u32,
            ItemKind::WaxedWeatheredCutCopper => 64u32,
            ItemKind::WaxedOxidizedCutCopper => 64u32,
            ItemKind::WaxedCutCopperStairs => 64u32,
            ItemKind::WaxedExposedCutCopperStairs => 64u32,
            ItemKind::WaxedWeatheredCutCopperStairs => 64u32,
            ItemKind::WaxedOxidizedCutCopperStairs => 64u32,
            ItemKind::WaxedCutCopperSlab => 64u32,
            ItemKind::WaxedExposedCutCopperSlab => 64u32,
            ItemKind::WaxedWeatheredCutCopperSlab => 64u32,
            ItemKind::WaxedOxidizedCutCopperSlab => 64u32,
            ItemKind::OakLog => 64u32,
            ItemKind::SpruceLog => 64u32,
            ItemKind::BirchLog => 64u32,
            ItemKind::JungleLog => 64u32,
            ItemKind::AcaciaLog => 64u32,
            ItemKind::DarkOakLog => 64u32,
            ItemKind::CrimsonStem => 64u32,
            ItemKind::WarpedStem => 64u32,
            ItemKind::StrippedOakLog => 64u32,
            ItemKind::StrippedSpruceLog => 64u32,
            ItemKind::StrippedBirchLog => 64u32,
            ItemKind::StrippedJungleLog => 64u32,
            ItemKind::StrippedAcaciaLog => 64u32,
            ItemKind::StrippedDarkOakLog => 64u32,
            ItemKind::StrippedCrimsonStem => 64u32,
            ItemKind::StrippedWarpedStem => 64u32,
            ItemKind::StrippedOakWood => 64u32,
            ItemKind::StrippedSpruceWood => 64u32,
            ItemKind::StrippedBirchWood => 64u32,
            ItemKind::StrippedJungleWood => 64u32,
            ItemKind::StrippedAcaciaWood => 64u32,
            ItemKind::StrippedDarkOakWood => 64u32,
            ItemKind::StrippedCrimsonHyphae => 64u32,
            ItemKind::StrippedWarpedHyphae => 64u32,
            ItemKind::OakWood => 64u32,
            ItemKind::SpruceWood => 64u32,
            ItemKind::BirchWood => 64u32,
            ItemKind::JungleWood => 64u32,
            ItemKind::AcaciaWood => 64u32,
            ItemKind::DarkOakWood => 64u32,
            ItemKind::CrimsonHyphae => 64u32,
            ItemKind::WarpedHyphae => 64u32,
            ItemKind::OakLeaves => 64u32,
            ItemKind::SpruceLeaves => 64u32,
            ItemKind::BirchLeaves => 64u32,
            ItemKind::JungleLeaves => 64u32,
            ItemKind::AcaciaLeaves => 64u32,
            ItemKind::DarkOakLeaves => 64u32,
            ItemKind::AzaleaLeaves => 64u32,
            ItemKind::FloweringAzaleaLeaves => 64u32,
            ItemKind::Sponge => 64u32,
            ItemKind::WetSponge => 64u32,
            ItemKind::Glass => 64u32,
            ItemKind::TintedGlass => 64u32,
            ItemKind::LapisBlock => 64u32,
            ItemKind::Sandstone => 64u32,
            ItemKind::ChiseledSandstone => 64u32,
            ItemKind::CutSandstone => 64u32,
            ItemKind::Cobweb => 64u32,
            ItemKind::Grass => 64u32,
            ItemKind::Fern => 64u32,
            ItemKind::Azalea => 64u32,
            ItemKind::FloweringAzalea => 64u32,
            ItemKind::DeadBush => 64u32,
            ItemKind::Seagrass => 64u32,
            ItemKind::SeaPickle => 64u32,
            ItemKind::WhiteWool => 64u32,
            ItemKind::OrangeWool => 64u32,
            ItemKind::MagentaWool => 64u32,
            ItemKind::LightBlueWool => 64u32,
            ItemKind::YellowWool => 64u32,
            ItemKind::LimeWool => 64u32,
            ItemKind::PinkWool => 64u32,
            ItemKind::GrayWool => 64u32,
            ItemKind::LightGrayWool => 64u32,
            ItemKind::CyanWool => 64u32,
            ItemKind::PurpleWool => 64u32,
            ItemKind::BlueWool => 64u32,
            ItemKind::BrownWool => 64u32,
            ItemKind::GreenWool => 64u32,
            ItemKind::RedWool => 64u32,
            ItemKind::BlackWool => 64u32,
            ItemKind::Dandelion => 64u32,
            ItemKind::Poppy => 64u32,
            ItemKind::BlueOrchid => 64u32,
            ItemKind::Allium => 64u32,
            ItemKind::AzureBluet => 64u32,
            ItemKind::RedTulip => 64u32,
            ItemKind::OrangeTulip => 64u32,
            ItemKind::WhiteTulip => 64u32,
            ItemKind::PinkTulip => 64u32,
            ItemKind::OxeyeDaisy => 64u32,
            ItemKind::Cornflower => 64u32,
            ItemKind::LilyOfTheValley => 64u32,
            ItemKind::WitherRose => 64u32,
            ItemKind::SporeBlossom => 64u32,
            ItemKind::BrownMushroom => 64u32,
            ItemKind::RedMushroom => 64u32,
            ItemKind::CrimsonFungus => 64u32,
            ItemKind::WarpedFungus => 64u32,
            ItemKind::CrimsonRoots => 64u32,
            ItemKind::WarpedRoots => 64u32,
            ItemKind::NetherSprouts => 64u32,
            ItemKind::WeepingVines => 64u32,
            ItemKind::TwistingVines => 64u32,
            ItemKind::SugarCane => 64u32,
            ItemKind::Kelp => 64u32,
            ItemKind::MossCarpet => 64u32,
            ItemKind::MossBlock => 64u32,
            ItemKind::HangingRoots => 64u32,
            ItemKind::BigDripleaf => 64u32,
            ItemKind::SmallDripleaf => 64u32,
            ItemKind::Bamboo => 64u32,
            ItemKind::OakSlab => 64u32,
            ItemKind::SpruceSlab => 64u32,
            ItemKind::BirchSlab => 64u32,
            ItemKind::JungleSlab => 64u32,
            ItemKind::AcaciaSlab => 64u32,
            ItemKind::DarkOakSlab => 64u32,
            ItemKind::CrimsonSlab => 64u32,
            ItemKind::WarpedSlab => 64u32,
            ItemKind::StoneSlab => 64u32,
            ItemKind::SmoothStoneSlab => 64u32,
            ItemKind::SandstoneSlab => 64u32,
            ItemKind::CutSandstoneSlab => 64u32,
            ItemKind::PetrifiedOakSlab => 64u32,
            ItemKind::CobblestoneSlab => 64u32,
            ItemKind::BrickSlab => 64u32,
            ItemKind::StoneBrickSlab => 64u32,
            ItemKind::NetherBrickSlab => 64u32,
            ItemKind::QuartzSlab => 64u32,
            ItemKind::RedSandstoneSlab => 64u32,
            ItemKind::CutRedSandstoneSlab => 64u32,
            ItemKind::PurpurSlab => 64u32,
            ItemKind::PrismarineSlab => 64u32,
            ItemKind::PrismarineBrickSlab => 64u32,
            ItemKind::DarkPrismarineSlab => 64u32,
            ItemKind::SmoothQuartz => 64u32,
            ItemKind::SmoothRedSandstone => 64u32,
            ItemKind::SmoothSandstone => 64u32,
            ItemKind::SmoothStone => 64u32,
            ItemKind::Bricks => 64u32,
            ItemKind::Bookshelf => 64u32,
            ItemKind::MossyCobblestone => 64u32,
            ItemKind::Obsidian => 64u32,
            ItemKind::Torch => 64u32,
            ItemKind::EndRod => 64u32,
            ItemKind::ChorusPlant => 64u32,
            ItemKind::ChorusFlower => 64u32,
            ItemKind::PurpurBlock => 64u32,
            ItemKind::PurpurPillar => 64u32,
            ItemKind::PurpurStairs => 64u32,
            ItemKind::Spawner => 64u32,
            ItemKind::OakStairs => 64u32,
            ItemKind::Chest => 64u32,
            ItemKind::CraftingTable => 64u32,
            ItemKind::Farmland => 64u32,
            ItemKind::Furnace => 64u32,
            ItemKind::Ladder => 64u32,
            ItemKind::CobblestoneStairs => 64u32,
            ItemKind::Snow => 64u32,
            ItemKind::Ice => 64u32,
            ItemKind::SnowBlock => 64u32,
            ItemKind::Cactus => 64u32,
            ItemKind::Clay => 64u32,
            ItemKind::Jukebox => 64u32,
            ItemKind::OakFence => 64u32,
            ItemKind::SpruceFence => 64u32,
            ItemKind::BirchFence => 64u32,
            ItemKind::JungleFence => 64u32,
            ItemKind::AcaciaFence => 64u32,
            ItemKind::DarkOakFence => 64u32,
            ItemKind::CrimsonFence => 64u32,
            ItemKind::WarpedFence => 64u32,
            ItemKind::Pumpkin => 64u32,
            ItemKind::CarvedPumpkin => 64u32,
            ItemKind::JackOLantern => 64u32,
            ItemKind::Netherrack => 64u32,
            ItemKind::SoulSand => 64u32,
            ItemKind::SoulSoil => 64u32,
            ItemKind::Basalt => 64u32,
            ItemKind::PolishedBasalt => 64u32,
            ItemKind::SmoothBasalt => 64u32,
            ItemKind::SoulTorch => 64u32,
            ItemKind::Glowstone => 64u32,
            ItemKind::InfestedStone => 64u32,
            ItemKind::InfestedCobblestone => 64u32,
            ItemKind::InfestedStoneBricks => 64u32,
            ItemKind::InfestedMossyStoneBricks => 64u32,
            ItemKind::InfestedCrackedStoneBricks => 64u32,
            ItemKind::InfestedChiseledStoneBricks => 64u32,
            ItemKind::InfestedDeepslate => 64u32,
            ItemKind::StoneBricks => 64u32,
            ItemKind::MossyStoneBricks => 64u32,
            ItemKind::CrackedStoneBricks => 64u32,
            ItemKind::ChiseledStoneBricks => 64u32,
            ItemKind::DeepslateBricks => 64u32,
            ItemKind::CrackedDeepslateBricks => 64u32,
            ItemKind::DeepslateTiles => 64u32,
            ItemKind::CrackedDeepslateTiles => 64u32,
            ItemKind::ChiseledDeepslate => 64u32,
            ItemKind::BrownMushroomBlock => 64u32,
            ItemKind::RedMushroomBlock => 64u32,
            ItemKind::MushroomStem => 64u32,
            ItemKind::IronBars => 64u32,
            ItemKind::Chain => 64u32,
            ItemKind::GlassPane => 64u32,
            ItemKind::Melon => 64u32,
            ItemKind::Vine => 64u32,
            ItemKind::GlowLichen => 64u32,
            ItemKind::BrickStairs => 64u32,
            ItemKind::StoneBrickStairs => 64u32,
            ItemKind::Mycelium => 64u32,
            ItemKind::LilyPad => 64u32,
            ItemKind::NetherBricks => 64u32,
            ItemKind::CrackedNetherBricks => 64u32,
            ItemKind::ChiseledNetherBricks => 64u32,
            ItemKind::NetherBrickFence => 64u32,
            ItemKind::NetherBrickStairs => 64u32,
            ItemKind::EnchantingTable => 64u32,
            ItemKind::EndPortalFrame => 64u32,
            ItemKind::EndStone => 64u32,
            ItemKind::EndStoneBricks => 64u32,
            ItemKind::DragonEgg => 64u32,
            ItemKind::SandstoneStairs => 64u32,
            ItemKind::EnderChest => 64u32,
            ItemKind::EmeraldBlock => 64u32,
            ItemKind::SpruceStairs => 64u32,
            ItemKind::BirchStairs => 64u32,
            ItemKind::JungleStairs => 64u32,
            ItemKind::CrimsonStairs => 64u32,
            ItemKind::WarpedStairs => 64u32,
            ItemKind::CommandBlock => 64u32,
            ItemKind::Beacon => 64u32,
            ItemKind::CobblestoneWall => 64u32,
            ItemKind::MossyCobblestoneWall => 64u32,
            ItemKind::BrickWall => 64u32,
            ItemKind::PrismarineWall => 64u32,
            ItemKind::RedSandstoneWall => 64u32,
            ItemKind::MossyStoneBrickWall => 64u32,
            ItemKind::GraniteWall => 64u32,
            ItemKind::StoneBrickWall => 64u32,
            ItemKind::NetherBrickWall => 64u32,
            ItemKind::AndesiteWall => 64u32,
            ItemKind::RedNetherBrickWall => 64u32,
            ItemKind::SandstoneWall => 64u32,
            ItemKind::EndStoneBrickWall => 64u32,
            ItemKind::DioriteWall => 64u32,
            ItemKind::BlackstoneWall => 64u32,
            ItemKind::PolishedBlackstoneWall => 64u32,
            ItemKind::PolishedBlackstoneBrickWall => 64u32,
            ItemKind::CobbledDeepslateWall => 64u32,
            ItemKind::PolishedDeepslateWall => 64u32,
            ItemKind::DeepslateBrickWall => 64u32,
            ItemKind::DeepslateTileWall => 64u32,
            ItemKind::Anvil => 64u32,
            ItemKind::ChippedAnvil => 64u32,
            ItemKind::DamagedAnvil => 64u32,
            ItemKind::ChiseledQuartzBlock => 64u32,
            ItemKind::QuartzBlock => 64u32,
            ItemKind::QuartzBricks => 64u32,
            ItemKind::QuartzPillar => 64u32,
            ItemKind::QuartzStairs => 64u32,
            ItemKind::WhiteTerracotta => 64u32,
            ItemKind::OrangeTerracotta => 64u32,
            ItemKind::MagentaTerracotta => 64u32,
            ItemKind::LightBlueTerracotta => 64u32,
            ItemKind::YellowTerracotta => 64u32,
            ItemKind::LimeTerracotta => 64u32,
            ItemKind::PinkTerracotta => 64u32,
            ItemKind::GrayTerracotta => 64u32,
            ItemKind::LightGrayTerracotta => 64u32,
            ItemKind::CyanTerracotta => 64u32,
            ItemKind::PurpleTerracotta => 64u32,
            ItemKind::BlueTerracotta => 64u32,
            ItemKind::BrownTerracotta => 64u32,
            ItemKind::GreenTerracotta => 64u32,
            ItemKind::RedTerracotta => 64u32,
            ItemKind::BlackTerracotta => 64u32,
            ItemKind::Barrier => 64u32,
            ItemKind::Light => 64u32,
            ItemKind::HayBlock => 64u32,
            ItemKind::WhiteCarpet => 64u32,
            ItemKind::OrangeCarpet => 64u32,
            ItemKind::MagentaCarpet => 64u32,
            ItemKind::LightBlueCarpet => 64u32,
            ItemKind::YellowCarpet => 64u32,
            ItemKind::LimeCarpet => 64u32,
            ItemKind::PinkCarpet => 64u32,
            ItemKind::GrayCarpet => 64u32,
            ItemKind::LightGrayCarpet => 64u32,
            ItemKind::CyanCarpet => 64u32,
            ItemKind::PurpleCarpet => 64u32,
            ItemKind::BlueCarpet => 64u32,
            ItemKind::BrownCarpet => 64u32,
            ItemKind::GreenCarpet => 64u32,
            ItemKind::RedCarpet => 64u32,
            ItemKind::BlackCarpet => 64u32,
            ItemKind::Terracotta => 64u32,
            ItemKind::PackedIce => 64u32,
            ItemKind::AcaciaStairs => 64u32,
            ItemKind::DarkOakStairs => 64u32,
            ItemKind::DirtPath => 64u32,
            ItemKind::Sunflower => 64u32,
            ItemKind::Lilac => 64u32,
            ItemKind::RoseBush => 64u32,
            ItemKind::Peony => 64u32,
            ItemKind::TallGrass => 64u32,
            ItemKind::LargeFern => 64u32,
            ItemKind::WhiteStainedGlass => 64u32,
            ItemKind::OrangeStainedGlass => 64u32,
            ItemKind::MagentaStainedGlass => 64u32,
            ItemKind::LightBlueStainedGlass => 64u32,
            ItemKind::YellowStainedGlass => 64u32,
            ItemKind::LimeStainedGlass => 64u32,
            ItemKind::PinkStainedGlass => 64u32,
            ItemKind::GrayStainedGlass => 64u32,
            ItemKind::LightGrayStainedGlass => 64u32,
            ItemKind::CyanStainedGlass => 64u32,
            ItemKind::PurpleStainedGlass => 64u32,
            ItemKind::BlueStainedGlass => 64u32,
            ItemKind::BrownStainedGlass => 64u32,
            ItemKind::GreenStainedGlass => 64u32,
            ItemKind::RedStainedGlass => 64u32,
            ItemKind::BlackStainedGlass => 64u32,
            ItemKind::WhiteStainedGlassPane => 64u32,
            ItemKind::OrangeStainedGlassPane => 64u32,
            ItemKind::MagentaStainedGlassPane => 64u32,
            ItemKind::LightBlueStainedGlassPane => 64u32,
            ItemKind::YellowStainedGlassPane => 64u32,
            ItemKind::LimeStainedGlassPane => 64u32,
            ItemKind::PinkStainedGlassPane => 64u32,
            ItemKind::GrayStainedGlassPane => 64u32,
            ItemKind::LightGrayStainedGlassPane => 64u32,
            ItemKind::CyanStainedGlassPane => 64u32,
            ItemKind::PurpleStainedGlassPane => 64u32,
            ItemKind::BlueStainedGlassPane => 64u32,
            ItemKind::BrownStainedGlassPane => 64u32,
            ItemKind::GreenStainedGlassPane => 64u32,
            ItemKind::RedStainedGlassPane => 64u32,
            ItemKind::BlackStainedGlassPane => 64u32,
            ItemKind::Prismarine => 64u32,
            ItemKind::PrismarineBricks => 64u32,
            ItemKind::DarkPrismarine => 64u32,
            ItemKind::PrismarineStairs => 64u32,
            ItemKind::PrismarineBrickStairs => 64u32,
            ItemKind::DarkPrismarineStairs => 64u32,
            ItemKind::SeaLantern => 64u32,
            ItemKind::RedSandstone => 64u32,
            ItemKind::ChiseledRedSandstone => 64u32,
            ItemKind::CutRedSandstone => 64u32,
            ItemKind::RedSandstoneStairs => 64u32,
            ItemKind::RepeatingCommandBlock => 64u32,
            ItemKind::ChainCommandBlock => 64u32,
            ItemKind::MagmaBlock => 64u32,
            ItemKind::NetherWartBlock => 64u32,
            ItemKind::WarpedWartBlock => 64u32,
            ItemKind::RedNetherBricks => 64u32,
            ItemKind::BoneBlock => 64u32,
            ItemKind::StructureVoid => 64u32,
            ItemKind::ShulkerBox => 1u32,
            ItemKind::WhiteShulkerBox => 1u32,
            ItemKind::OrangeShulkerBox => 1u32,
            ItemKind::MagentaShulkerBox => 1u32,
            ItemKind::LightBlueShulkerBox => 1u32,
            ItemKind::YellowShulkerBox => 1u32,
            ItemKind::LimeShulkerBox => 1u32,
            ItemKind::PinkShulkerBox => 1u32,
            ItemKind::GrayShulkerBox => 1u32,
            ItemKind::LightGrayShulkerBox => 1u32,
            ItemKind::CyanShulkerBox => 1u32,
            ItemKind::PurpleShulkerBox => 1u32,
            ItemKind::BlueShulkerBox => 1u32,
            ItemKind::BrownShulkerBox => 1u32,
            ItemKind::GreenShulkerBox => 1u32,
            ItemKind::RedShulkerBox => 1u32,
            ItemKind::BlackShulkerBox => 1u32,
            ItemKind::WhiteGlazedTerracotta => 64u32,
            ItemKind::OrangeGlazedTerracotta => 64u32,
            ItemKind::MagentaGlazedTerracotta => 64u32,
            ItemKind::LightBlueGlazedTerracotta => 64u32,
            ItemKind::YellowGlazedTerracotta => 64u32,
            ItemKind::LimeGlazedTerracotta => 64u32,
            ItemKind::PinkGlazedTerracotta => 64u32,
            ItemKind::GrayGlazedTerracotta => 64u32,
            ItemKind::LightGrayGlazedTerracotta => 64u32,
            ItemKind::CyanGlazedTerracotta => 64u32,
            ItemKind::PurpleGlazedTerracotta => 64u32,
            ItemKind::BlueGlazedTerracotta => 64u32,
            ItemKind::BrownGlazedTerracotta => 64u32,
            ItemKind::GreenGlazedTerracotta => 64u32,
            ItemKind::RedGlazedTerracotta => 64u32,
            ItemKind::BlackGlazedTerracotta => 64u32,
            ItemKind::WhiteConcrete => 64u32,
            ItemKind::OrangeConcrete => 64u32,
            ItemKind::MagentaConcrete => 64u32,
            ItemKind::LightBlueConcrete => 64u32,
            ItemKind::YellowConcrete => 64u32,
            ItemKind::LimeConcrete => 64u32,
            ItemKind::PinkConcrete => 64u32,
            ItemKind::GrayConcrete => 64u32,
            ItemKind::LightGrayConcrete => 64u32,
            ItemKind::CyanConcrete => 64u32,
            ItemKind::PurpleConcrete => 64u32,
            ItemKind::BlueConcrete => 64u32,
            ItemKind::BrownConcrete => 64u32,
            ItemKind::GreenConcrete => 64u32,
            ItemKind::RedConcrete => 64u32,
            ItemKind::BlackConcrete => 64u32,
            ItemKind::WhiteConcretePowder => 64u32,
            ItemKind::OrangeConcretePowder => 64u32,
            ItemKind::MagentaConcretePowder => 64u32,
            ItemKind::LightBlueConcretePowder => 64u32,
            ItemKind::YellowConcretePowder => 64u32,
            ItemKind::LimeConcretePowder => 64u32,
            ItemKind::PinkConcretePowder => 64u32,
            ItemKind::GrayConcretePowder => 64u32,
            ItemKind::LightGrayConcretePowder => 64u32,
            ItemKind::CyanConcretePowder => 64u32,
            ItemKind::PurpleConcretePowder => 64u32,
            ItemKind::BlueConcretePowder => 64u32,
            ItemKind::BrownConcretePowder => 64u32,
            ItemKind::GreenConcretePowder => 64u32,
            ItemKind::RedConcretePowder => 64u32,
            ItemKind::BlackConcretePowder => 64u32,
            ItemKind::TurtleEgg => 64u32,
            ItemKind::DeadTubeCoralBlock => 64u32,
            ItemKind::DeadBrainCoralBlock => 64u32,
            ItemKind::DeadBubbleCoralBlock => 64u32,
            ItemKind::DeadFireCoralBlock => 64u32,
            ItemKind::DeadHornCoralBlock => 64u32,
            ItemKind::TubeCoralBlock => 64u32,
            ItemKind::BrainCoralBlock => 64u32,
            ItemKind::BubbleCoralBlock => 64u32,
            ItemKind::FireCoralBlock => 64u32,
            ItemKind::HornCoralBlock => 64u32,
            ItemKind::TubeCoral => 64u32,
            ItemKind::BrainCoral => 64u32,
            ItemKind::BubbleCoral => 64u32,
            ItemKind::FireCoral => 64u32,
            ItemKind::HornCoral => 64u32,
            ItemKind::DeadBrainCoral => 64u32,
            ItemKind::DeadBubbleCoral => 64u32,
            ItemKind::DeadFireCoral => 64u32,
            ItemKind::DeadHornCoral => 64u32,
            ItemKind::DeadTubeCoral => 64u32,
            ItemKind::TubeCoralFan => 64u32,
            ItemKind::BrainCoralFan => 64u32,
            ItemKind::BubbleCoralFan => 64u32,
            ItemKind::FireCoralFan => 64u32,
            ItemKind::HornCoralFan => 64u32,
            ItemKind::DeadTubeCoralFan => 64u32,
            ItemKind::DeadBrainCoralFan => 64u32,
            ItemKind::DeadBubbleCoralFan => 64u32,
            ItemKind::DeadFireCoralFan => 64u32,
            ItemKind::DeadHornCoralFan => 64u32,
            ItemKind::BlueIce => 64u32,
            ItemKind::Conduit => 64u32,
            ItemKind::PolishedGraniteStairs => 64u32,
            ItemKind::SmoothRedSandstoneStairs => 64u32,
            ItemKind::MossyStoneBrickStairs => 64u32,
            ItemKind::PolishedDioriteStairs => 64u32,
            ItemKind::MossyCobblestoneStairs => 64u32,
            ItemKind::EndStoneBrickStairs => 64u32,
            ItemKind::StoneStairs => 64u32,
            ItemKind::SmoothSandstoneStairs => 64u32,
            ItemKind::SmoothQuartzStairs => 64u32,
            ItemKind::GraniteStairs => 64u32,
            ItemKind::AndesiteStairs => 64u32,
            ItemKind::RedNetherBrickStairs => 64u32,
            ItemKind::PolishedAndesiteStairs => 64u32,
            ItemKind::DioriteStairs => 64u32,
            ItemKind::CobbledDeepslateStairs => 64u32,
            ItemKind::PolishedDeepslateStairs => 64u32,
            ItemKind::DeepslateBrickStairs => 64u32,
            ItemKind::DeepslateTileStairs => 64u32,
            ItemKind::PolishedGraniteSlab => 64u32,
            ItemKind::SmoothRedSandstoneSlab => 64u32,
            ItemKind::MossyStoneBrickSlab => 64u32,
            ItemKind::PolishedDioriteSlab => 64u32,
            ItemKind::MossyCobblestoneSlab => 64u32,
            ItemKind::EndStoneBrickSlab => 64u32,
            ItemKind::SmoothSandstoneSlab => 64u32,
            ItemKind::SmoothQuartzSlab => 64u32,
            ItemKind::GraniteSlab => 64u32,
            ItemKind::AndesiteSlab => 64u32,
            ItemKind::RedNetherBrickSlab => 64u32,
            ItemKind::PolishedAndesiteSlab => 64u32,
            ItemKind::DioriteSlab => 64u32,
            ItemKind::CobbledDeepslateSlab => 64u32,
            ItemKind::PolishedDeepslateSlab => 64u32,
            ItemKind::DeepslateBrickSlab => 64u32,
            ItemKind::DeepslateTileSlab => 64u32,
            ItemKind::Scaffolding => 64u32,
            ItemKind::Redstone => 64u32,
            ItemKind::RedstoneTorch => 64u32,
            ItemKind::RedstoneBlock => 64u32,
            ItemKind::Repeater => 64u32,
            ItemKind::Comparator => 64u32,
            ItemKind::Piston => 64u32,
            ItemKind::StickyPiston => 64u32,
            ItemKind::SlimeBlock => 64u32,
            ItemKind::HoneyBlock => 64u32,
            ItemKind::Observer => 64u32,
            ItemKind::Hopper => 64u32,
            ItemKind::Dispenser => 64u32,
            ItemKind::Dropper => 64u32,
            ItemKind::Lectern => 64u32,
            ItemKind::Target => 64u32,
            ItemKind::Lever => 64u32,
            ItemKind::LightningRod => 64u32,
            ItemKind::DaylightDetector => 64u32,
            ItemKind::SculkSensor => 64u32,
            ItemKind::TripwireHook => 64u32,
            ItemKind::TrappedChest => 64u32,
            ItemKind::Tnt => 64u32,
            ItemKind::RedstoneLamp => 64u32,
            ItemKind::NoteBlock => 64u32,
            ItemKind::StoneButton => 64u32,
            ItemKind::PolishedBlackstoneButton => 64u32,
            ItemKind::OakButton => 64u32,
            ItemKind::SpruceButton => 64u32,
            ItemKind::BirchButton => 64u32,
            ItemKind::JungleButton => 64u32,
            ItemKind::AcaciaButton => 64u32,
            ItemKind::DarkOakButton => 64u32,
            ItemKind::CrimsonButton => 64u32,
            ItemKind::WarpedButton => 64u32,
            ItemKind::StonePressurePlate => 64u32,
            ItemKind::PolishedBlackstonePressurePlate => 64u32,
            ItemKind::LightWeightedPressurePlate => 64u32,
            ItemKind::HeavyWeightedPressurePlate => 64u32,
            ItemKind::OakPressurePlate => 64u32,
            ItemKind::SprucePressurePlate => 64u32,
            ItemKind::BirchPressurePlate => 64u32,
            ItemKind::JunglePressurePlate => 64u32,
            ItemKind::AcaciaPressurePlate => 64u32,
            ItemKind::DarkOakPressurePlate => 64u32,
            ItemKind::CrimsonPressurePlate => 64u32,
            ItemKind::WarpedPressurePlate => 64u32,
            ItemKind::IronDoor => 64u32,
            ItemKind::OakDoor => 64u32,
            ItemKind::SpruceDoor => 64u32,
            ItemKind::BirchDoor => 64u32,
            ItemKind::JungleDoor => 64u32,
            ItemKind::AcaciaDoor => 64u32,
            ItemKind::DarkOakDoor => 64u32,
            ItemKind::CrimsonDoor => 64u32,
            ItemKind::WarpedDoor => 64u32,
            ItemKind::IronTrapdoor => 64u32,
            ItemKind::OakTrapdoor => 64u32,
            ItemKind::SpruceTrapdoor => 64u32,
            ItemKind::BirchTrapdoor => 64u32,
            ItemKind::JungleTrapdoor => 64u32,
            ItemKind::AcaciaTrapdoor => 64u32,
            ItemKind::DarkOakTrapdoor => 64u32,
            ItemKind::CrimsonTrapdoor => 64u32,
            ItemKind::WarpedTrapdoor => 64u32,
            ItemKind::OakFenceGate => 64u32,
            ItemKind::SpruceFenceGate => 64u32,
            ItemKind::BirchFenceGate => 64u32,
            ItemKind::JungleFenceGate => 64u32,
            ItemKind::AcaciaFenceGate => 64u32,
            ItemKind::DarkOakFenceGate => 64u32,
            ItemKind::CrimsonFenceGate => 64u32,
            ItemKind::WarpedFenceGate => 64u32,
            ItemKind::PoweredRail => 64u32,
            ItemKind::DetectorRail => 64u32,
            ItemKind::Rail => 64u32,
            ItemKind::ActivatorRail => 64u32,
            ItemKind::Saddle => 1u32,
            ItemKind::Minecart => 1u32,
            ItemKind::ChestMinecart => 1u32,
            ItemKind::FurnaceMinecart => 1u32,
            ItemKind::TntMinecart => 1u32,
            ItemKind::HopperMinecart => 1u32,
            ItemKind::CarrotOnAStick => 1u32,
            ItemKind::WarpedFungusOnAStick => 64u32,
            ItemKind::Elytra => 1u32,
            ItemKind::OakBoat => 1u32,
            ItemKind::SpruceBoat => 1u32,
            ItemKind::BirchBoat => 1u32,
            ItemKind::JungleBoat => 1u32,
            ItemKind::AcaciaBoat => 1u32,
            ItemKind::DarkOakBoat => 1u32,
            ItemKind::StructureBlock => 64u32,
            ItemKind::Jigsaw => 64u32,
            ItemKind::TurtleHelmet => 1u32,
            ItemKind::Scute => 64u32,
            ItemKind::FlintAndSteel => 1u32,
            ItemKind::Apple => 64u32,
            ItemKind::Bow => 1u32,
            ItemKind::Arrow => 64u32,
            ItemKind::Coal => 64u32,
            ItemKind::Charcoal => 64u32,
            ItemKind::Diamond => 64u32,
            ItemKind::Emerald => 64u32,
            ItemKind::LapisLazuli => 64u32,
            ItemKind::Quartz => 64u32,
            ItemKind::AmethystShard => 64u32,
            ItemKind::RawIron => 64u32,
            ItemKind::IronIngot => 64u32,
            ItemKind::RawCopper => 64u32,
            ItemKind::CopperIngot => 64u32,
            ItemKind::RawGold => 64u32,
            ItemKind::GoldIngot => 64u32,
            ItemKind::NetheriteIngot => 64u32,
            ItemKind::NetheriteScrap => 64u32,
            ItemKind::WoodenSword => 1u32,
            ItemKind::WoodenShovel => 1u32,
            ItemKind::WoodenPickaxe => 1u32,
            ItemKind::WoodenAxe => 1u32,
            ItemKind::WoodenHoe => 1u32,
            ItemKind::StoneSword => 1u32,
            ItemKind::StoneShovel => 1u32,
            ItemKind::StonePickaxe => 1u32,
            ItemKind::StoneAxe => 1u32,
            ItemKind::StoneHoe => 1u32,
            ItemKind::GoldenSword => 1u32,
            ItemKind::GoldenShovel => 1u32,
            ItemKind::GoldenPickaxe => 1u32,
            ItemKind::GoldenAxe => 1u32,
            ItemKind::GoldenHoe => 1u32,
            ItemKind::IronSword => 1u32,
            ItemKind::IronShovel => 1u32,
            ItemKind::IronPickaxe => 1u32,
            ItemKind::IronAxe => 1u32,
            ItemKind::IronHoe => 1u32,
            ItemKind::DiamondSword => 1u32,
            ItemKind::DiamondShovel => 1u32,
            ItemKind::DiamondPickaxe => 1u32,
            ItemKind::DiamondAxe => 1u32,
            ItemKind::DiamondHoe => 1u32,
            ItemKind::NetheriteSword => 1u32,
            ItemKind::NetheriteShovel => 1u32,
            ItemKind::NetheritePickaxe => 1u32,
            ItemKind::NetheriteAxe => 1u32,
            ItemKind::NetheriteHoe => 1u32,
            ItemKind::Stick => 64u32,
            ItemKind::Bowl => 64u32,
            ItemKind::MushroomStew => 1u32,
            ItemKind::String => 64u32,
            ItemKind::Feather => 64u32,
            ItemKind::Gunpowder => 64u32,
            ItemKind::WheatSeeds => 64u32,
            ItemKind::Wheat => 64u32,
            ItemKind::Bread => 64u32,
            ItemKind::LeatherHelmet => 1u32,
            ItemKind::LeatherChestplate => 1u32,
            ItemKind::LeatherLeggings => 1u32,
            ItemKind::LeatherBoots => 1u32,
            ItemKind::ChainmailHelmet => 1u32,
            ItemKind::ChainmailChestplate => 1u32,
            ItemKind::ChainmailLeggings => 1u32,
            ItemKind::ChainmailBoots => 1u32,
            ItemKind::IronHelmet => 1u32,
            ItemKind::IronChestplate => 1u32,
            ItemKind::IronLeggings => 1u32,
            ItemKind::IronBoots => 1u32,
            ItemKind::DiamondHelmet => 1u32,
            ItemKind::DiamondChestplate => 1u32,
            ItemKind::DiamondLeggings => 1u32,
            ItemKind::DiamondBoots => 1u32,
            ItemKind::GoldenHelmet => 1u32,
            ItemKind::GoldenChestplate => 1u32,
            ItemKind::GoldenLeggings => 1u32,
            ItemKind::GoldenBoots => 1u32,
            ItemKind::NetheriteHelmet => 1u32,
            ItemKind::NetheriteChestplate => 1u32,
            ItemKind::NetheriteLeggings => 1u32,
            ItemKind::NetheriteBoots => 1u32,
            ItemKind::Flint => 64u32,
            ItemKind::Porkchop => 64u32,
            ItemKind::CookedPorkchop => 64u32,
            ItemKind::Painting => 64u32,
            ItemKind::GoldenApple => 64u32,
            ItemKind::EnchantedGoldenApple => 64u32,
            ItemKind::OakSign => 16u32,
            ItemKind::SpruceSign => 16u32,
            ItemKind::BirchSign => 16u32,
            ItemKind::JungleSign => 16u32,
            ItemKind::AcaciaSign => 16u32,
            ItemKind::DarkOakSign => 16u32,
            ItemKind::CrimsonSign => 16u32,
            ItemKind::WarpedSign => 16u32,
            ItemKind::Bucket => 16u32,
            ItemKind::WaterBucket => 1u32,
            ItemKind::LavaBucket => 1u32,
            ItemKind::PowderSnowBucket => 1u32,
            ItemKind::Snowball => 16u32,
            ItemKind::Leather => 64u32,
            ItemKind::MilkBucket => 1u32,
            ItemKind::PufferfishBucket => 1u32,
            ItemKind::SalmonBucket => 1u32,
            ItemKind::CodBucket => 1u32,
            ItemKind::TropicalFishBucket => 1u32,
            ItemKind::AxolotlBucket => 1u32,
            ItemKind::Brick => 64u32,
            ItemKind::ClayBall => 64u32,
            ItemKind::DriedKelpBlock => 64u32,
            ItemKind::Paper => 64u32,
            ItemKind::Book => 64u32,
            ItemKind::SlimeBall => 64u32,
            ItemKind::Egg => 16u32,
            ItemKind::Compass => 64u32,
            ItemKind::Bundle => 1u32,
            ItemKind::FishingRod => 1u32,
            ItemKind::Clock => 64u32,
            ItemKind::Spyglass => 1u32,
            ItemKind::GlowstoneDust => 64u32,
            ItemKind::Cod => 64u32,
            ItemKind::Salmon => 64u32,
            ItemKind::TropicalFish => 64u32,
            ItemKind::Pufferfish => 64u32,
            ItemKind::CookedCod => 64u32,
            ItemKind::CookedSalmon => 64u32,
            ItemKind::InkSac => 64u32,
            ItemKind::GlowInkSac => 64u32,
            ItemKind::CocoaBeans => 64u32,
            ItemKind::WhiteDye => 64u32,
            ItemKind::OrangeDye => 64u32,
            ItemKind::MagentaDye => 64u32,
            ItemKind::LightBlueDye => 64u32,
            ItemKind::YellowDye => 64u32,
            ItemKind::LimeDye => 64u32,
            ItemKind::PinkDye => 64u32,
            ItemKind::GrayDye => 64u32,
            ItemKind::LightGrayDye => 64u32,
            ItemKind::CyanDye => 64u32,
            ItemKind::PurpleDye => 64u32,
            ItemKind::BlueDye => 64u32,
            ItemKind::BrownDye => 64u32,
            ItemKind::GreenDye => 64u32,
            ItemKind::RedDye => 64u32,
            ItemKind::BlackDye => 64u32,
            ItemKind::BoneMeal => 64u32,
            ItemKind::Bone => 64u32,
            ItemKind::Sugar => 64u32,
            ItemKind::Cake => 1u32,
            ItemKind::WhiteBed => 1u32,
            ItemKind::OrangeBed => 1u32,
            ItemKind::MagentaBed => 1u32,
            ItemKind::LightBlueBed => 1u32,
            ItemKind::YellowBed => 1u32,
            ItemKind::LimeBed => 1u32,
            ItemKind::PinkBed => 1u32,
            ItemKind::GrayBed => 1u32,
            ItemKind::LightGrayBed => 1u32,
            ItemKind::CyanBed => 1u32,
            ItemKind::PurpleBed => 1u32,
            ItemKind::BlueBed => 1u32,
            ItemKind::BrownBed => 1u32,
            ItemKind::GreenBed => 1u32,
            ItemKind::RedBed => 1u32,
            ItemKind::BlackBed => 1u32,
            ItemKind::Cookie => 64u32,
            ItemKind::FilledMap => 64u32,
            ItemKind::Shears => 1u32,
            ItemKind::MelonSlice => 64u32,
            ItemKind::DriedKelp => 64u32,
            ItemKind::PumpkinSeeds => 64u32,
            ItemKind::MelonSeeds => 64u32,
            ItemKind::Beef => 64u32,
            ItemKind::CookedBeef => 64u32,
            ItemKind::Chicken => 64u32,
            ItemKind::CookedChicken => 64u32,
            ItemKind::RottenFlesh => 64u32,
            ItemKind::EnderPearl => 16u32,
            ItemKind::BlazeRod => 64u32,
            ItemKind::GhastTear => 64u32,
            ItemKind::GoldNugget => 64u32,
            ItemKind::NetherWart => 64u32,
            ItemKind::Potion => 1u32,
            ItemKind::GlassBottle => 64u32,
            ItemKind::SpiderEye => 64u32,
            ItemKind::FermentedSpiderEye => 64u32,
            ItemKind::BlazePowder => 64u32,
            ItemKind::MagmaCream => 64u32,
            ItemKind::BrewingStand => 64u32,
            ItemKind::Cauldron => 64u32,
            ItemKind::EnderEye => 64u32,
            ItemKind::GlisteringMelonSlice => 64u32,
            ItemKind::AxolotlSpawnEgg => 64u32,
            ItemKind::BatSpawnEgg => 64u32,
            ItemKind::BeeSpawnEgg => 64u32,
            ItemKind::BlazeSpawnEgg => 64u32,
            ItemKind::CatSpawnEgg => 64u32,
            ItemKind::CaveSpiderSpawnEgg => 64u32,
            ItemKind::ChickenSpawnEgg => 64u32,
            ItemKind::CodSpawnEgg => 64u32,
            ItemKind::CowSpawnEgg => 64u32,
            ItemKind::CreeperSpawnEgg => 64u32,
            ItemKind::DolphinSpawnEgg => 64u32,
            ItemKind::DonkeySpawnEgg => 64u32,
            ItemKind::DrownedSpawnEgg => 64u32,
            ItemKind::ElderGuardianSpawnEgg => 64u32,
            ItemKind::EndermanSpawnEgg => 64u32,
            ItemKind::EndermiteSpawnEgg => 64u32,
            ItemKind::EvokerSpawnEgg => 64u32,
            ItemKind::FoxSpawnEgg => 64u32,
            ItemKind::GhastSpawnEgg => 64u32,
            ItemKind::GlowSquidSpawnEgg => 64u32,
            ItemKind::GoatSpawnEgg => 64u32,
            ItemKind::GuardianSpawnEgg => 64u32,
            ItemKind::HoglinSpawnEgg => 64u32,
            ItemKind::HorseSpawnEgg => 64u32,
            ItemKind::HuskSpawnEgg => 64u32,
            ItemKind::LlamaSpawnEgg => 64u32,
            ItemKind::MagmaCubeSpawnEgg => 64u32,
            ItemKind::MooshroomSpawnEgg => 64u32,
            ItemKind::MuleSpawnEgg => 64u32,
            ItemKind::OcelotSpawnEgg => 64u32,
            ItemKind::PandaSpawnEgg => 64u32,
            ItemKind::ParrotSpawnEgg => 64u32,
            ItemKind::PhantomSpawnEgg => 64u32,
            ItemKind::PigSpawnEgg => 64u32,
            ItemKind::PiglinSpawnEgg => 64u32,
            ItemKind::PiglinBruteSpawnEgg => 64u32,
            ItemKind::PillagerSpawnEgg => 64u32,
            ItemKind::PolarBearSpawnEgg => 64u32,
            ItemKind::PufferfishSpawnEgg => 64u32,
            ItemKind::RabbitSpawnEgg => 64u32,
            ItemKind::RavagerSpawnEgg => 64u32,
            ItemKind::SalmonSpawnEgg => 64u32,
            ItemKind::SheepSpawnEgg => 64u32,
            ItemKind::ShulkerSpawnEgg => 64u32,
            ItemKind::SilverfishSpawnEgg => 64u32,
            ItemKind::SkeletonSpawnEgg => 64u32,
            ItemKind::SkeletonHorseSpawnEgg => 64u32,
            ItemKind::SlimeSpawnEgg => 64u32,
            ItemKind::SpiderSpawnEgg => 64u32,
            ItemKind::SquidSpawnEgg => 64u32,
            ItemKind::StraySpawnEgg => 64u32,
            ItemKind::StriderSpawnEgg => 64u32,
            ItemKind::TraderLlamaSpawnEgg => 64u32,
            ItemKind::TropicalFishSpawnEgg => 64u32,
            ItemKind::TurtleSpawnEgg => 64u32,
            ItemKind::VexSpawnEgg => 64u32,
            ItemKind::VillagerSpawnEgg => 64u32,
            ItemKind::VindicatorSpawnEgg => 64u32,
            ItemKind::WanderingTraderSpawnEgg => 64u32,
            ItemKind::WitchSpawnEgg => 64u32,
            ItemKind::WitherSkeletonSpawnEgg => 64u32,
            ItemKind::WolfSpawnEgg => 64u32,
            ItemKind::ZoglinSpawnEgg => 64u32,
            ItemKind::ZombieSpawnEgg => 64u32,
            ItemKind::ZombieHorseSpawnEgg => 64u32,
            ItemKind::ZombieVillagerSpawnEgg => 64u32,
            ItemKind::ZombifiedPiglinSpawnEgg => 64u32,
            ItemKind::ExperienceBottle => 64u32,
            ItemKind::FireCharge => 64u32,
            ItemKind::WritableBook => 1u32,
            ItemKind::WrittenBook => 16u32,
            ItemKind::ItemFrame => 64u32,
            ItemKind::GlowItemFrame => 64u32,
            ItemKind::FlowerPot => 64u32,
            ItemKind::Carrot => 64u32,
            ItemKind::Potato => 64u32,
            ItemKind::BakedPotato => 64u32,
            ItemKind::PoisonousPotato => 64u32,
            ItemKind::Map => 64u32,
            ItemKind::GoldenCarrot => 64u32,
            ItemKind::SkeletonSkull => 64u32,
            ItemKind::WitherSkeletonSkull => 64u32,
            ItemKind::PlayerHead => 64u32,
            ItemKind::ZombieHead => 64u32,
            ItemKind::CreeperHead => 64u32,
            ItemKind::DragonHead => 64u32,
            ItemKind::NetherStar => 64u32,
            ItemKind::PumpkinPie => 64u32,
            ItemKind::FireworkRocket => 64u32,
            ItemKind::FireworkStar => 64u32,
            ItemKind::EnchantedBook => 1u32,
            ItemKind::NetherBrick => 64u32,
            ItemKind::PrismarineShard => 64u32,
            ItemKind::PrismarineCrystals => 64u32,
            ItemKind::Rabbit => 64u32,
            ItemKind::CookedRabbit => 64u32,
            ItemKind::RabbitStew => 1u32,
            ItemKind::RabbitFoot => 64u32,
            ItemKind::RabbitHide => 64u32,
            ItemKind::ArmorStand => 16u32,
            ItemKind::IronHorseArmor => 1u32,
            ItemKind::GoldenHorseArmor => 1u32,
            ItemKind::DiamondHorseArmor => 1u32,
            ItemKind::LeatherHorseArmor => 1u32,
            ItemKind::Lead => 64u32,
            ItemKind::NameTag => 64u32,
            ItemKind::CommandBlockMinecart => 1u32,
            ItemKind::Mutton => 64u32,
            ItemKind::CookedMutton => 64u32,
            ItemKind::WhiteBanner => 16u32,
            ItemKind::OrangeBanner => 16u32,
            ItemKind::MagentaBanner => 16u32,
            ItemKind::LightBlueBanner => 16u32,
            ItemKind::YellowBanner => 16u32,
            ItemKind::LimeBanner => 16u32,
            ItemKind::PinkBanner => 16u32,
            ItemKind::GrayBanner => 16u32,
            ItemKind::LightGrayBanner => 16u32,
            ItemKind::CyanBanner => 16u32,
            ItemKind::PurpleBanner => 16u32,
            ItemKind::BlueBanner => 16u32,
            ItemKind::BrownBanner => 16u32,
            ItemKind::GreenBanner => 16u32,
            ItemKind::RedBanner => 16u32,
            ItemKind::BlackBanner => 16u32,
            ItemKind::EndCrystal => 64u32,
            ItemKind::ChorusFruit => 64u32,
            ItemKind::PoppedChorusFruit => 64u32,
            ItemKind::Beetroot => 64u32,
            ItemKind::BeetrootSeeds => 64u32,
            ItemKind::BeetrootSoup => 1u32,
            ItemKind::DragonBreath => 64u32,
            ItemKind::SplashPotion => 1u32,
            ItemKind::SpectralArrow => 64u32,
            ItemKind::TippedArrow => 64u32,
            ItemKind::LingeringPotion => 1u32,
            ItemKind::Shield => 1u32,
            ItemKind::TotemOfUndying => 1u32,
            ItemKind::ShulkerShell => 64u32,
            ItemKind::IronNugget => 64u32,
            ItemKind::KnowledgeBook => 1u32,
            ItemKind::DebugStick => 1u32,
            ItemKind::MusicDisc13 => 1u32,
            ItemKind::MusicDiscCat => 1u32,
            ItemKind::MusicDiscBlocks => 1u32,
            ItemKind::MusicDiscChirp => 1u32,
            ItemKind::MusicDiscFar => 1u32,
            ItemKind::MusicDiscMall => 1u32,
            ItemKind::MusicDiscMellohi => 1u32,
            ItemKind::MusicDiscStal => 1u32,
            ItemKind::MusicDiscStrad => 1u32,
            ItemKind::MusicDiscWard => 1u32,
            ItemKind::MusicDisc11 => 1u32,
            ItemKind::MusicDiscWait => 1u32,
            ItemKind::MusicDiscOtherside => 1u32,
            ItemKind::MusicDiscPigstep => 1u32,
            ItemKind::Trident => 1u32,
            ItemKind::PhantomMembrane => 64u32,
            ItemKind::NautilusShell => 64u32,
            ItemKind::HeartOfTheSea => 64u32,
            ItemKind::Crossbow => 1u32,
            ItemKind::SuspiciousStew => 1u32,
            ItemKind::Loom => 64u32,
            ItemKind::FlowerBannerPattern => 1u32,
            ItemKind::CreeperBannerPattern => 1u32,
            ItemKind::SkullBannerPattern => 1u32,
            ItemKind::MojangBannerPattern => 1u32,
            ItemKind::GlobeBannerPattern => 1u32,
            ItemKind::PiglinBannerPattern => 1u32,
            ItemKind::Composter => 64u32,
            ItemKind::Barrel => 64u32,
            ItemKind::Smoker => 64u32,
            ItemKind::BlastFurnace => 64u32,
            ItemKind::CartographyTable => 64u32,
            ItemKind::FletchingTable => 64u32,
            ItemKind::Grindstone => 64u32,
            ItemKind::SmithingTable => 64u32,
            ItemKind::Stonecutter => 64u32,
            ItemKind::Bell => 64u32,
            ItemKind::Lantern => 64u32,
            ItemKind::SoulLantern => 64u32,
            ItemKind::SweetBerries => 64u32,
            ItemKind::GlowBerries => 64u32,
            ItemKind::Campfire => 64u32,
            ItemKind::SoulCampfire => 64u32,
            ItemKind::Shroomlight => 64u32,
            ItemKind::Honeycomb => 64u32,
            ItemKind::BeeNest => 64u32,
            ItemKind::Beehive => 64u32,
            ItemKind::HoneyBottle => 16u32,
            ItemKind::HoneycombBlock => 64u32,
            ItemKind::Lodestone => 64u32,
            ItemKind::CryingObsidian => 64u32,
            ItemKind::Blackstone => 64u32,
            ItemKind::BlackstoneSlab => 64u32,
            ItemKind::BlackstoneStairs => 64u32,
            ItemKind::GildedBlackstone => 64u32,
            ItemKind::PolishedBlackstone => 64u32,
            ItemKind::PolishedBlackstoneSlab => 64u32,
            ItemKind::PolishedBlackstoneStairs => 64u32,
            ItemKind::ChiseledPolishedBlackstone => 64u32,
            ItemKind::PolishedBlackstoneBricks => 64u32,
            ItemKind::PolishedBlackstoneBrickSlab => 64u32,
            ItemKind::PolishedBlackstoneBrickStairs => 64u32,
            ItemKind::CrackedPolishedBlackstoneBricks => 64u32,
            ItemKind::RespawnAnchor => 64u32,
            ItemKind::Candle => 64u32,
            ItemKind::WhiteCandle => 64u32,
            ItemKind::OrangeCandle => 64u32,
            ItemKind::MagentaCandle => 64u32,
            ItemKind::LightBlueCandle => 64u32,
            ItemKind::YellowCandle => 64u32,
            ItemKind::LimeCandle => 64u32,
            ItemKind::PinkCandle => 64u32,
            ItemKind::GrayCandle => 64u32,
            ItemKind::LightGrayCandle => 64u32,
            ItemKind::CyanCandle => 64u32,
            ItemKind::PurpleCandle => 64u32,
            ItemKind::BlueCandle => 64u32,
            ItemKind::BrownCandle => 64u32,
            ItemKind::GreenCandle => 64u32,
            ItemKind::RedCandle => 64u32,
            ItemKind::BlackCandle => 64u32,
            ItemKind::SmallAmethystBud => 64u32,
            ItemKind::MediumAmethystBud => 64u32,
            ItemKind::LargeAmethystBud => 64u32,
            ItemKind::AmethystCluster => 64u32,
            ItemKind::PointedDripstone => 64u32,
        }
    }
}
impl ItemKind {
    #[doc = "Returns the `max_durability` property of this `ItemKind`."]
    #[inline]
    pub const fn max_durability(&self) -> Option<u32> {
        match self {
            ItemKind::Stone => None,
            ItemKind::Granite => None,
            ItemKind::PolishedGranite => None,
            ItemKind::Diorite => None,
            ItemKind::PolishedDiorite => None,
            ItemKind::Andesite => None,
            ItemKind::PolishedAndesite => None,
            ItemKind::Deepslate => None,
            ItemKind::CobbledDeepslate => None,
            ItemKind::PolishedDeepslate => None,
            ItemKind::Calcite => None,
            ItemKind::Tuff => None,
            ItemKind::DripstoneBlock => None,
            ItemKind::GrassBlock => None,
            ItemKind::Dirt => None,
            ItemKind::CoarseDirt => None,
            ItemKind::Podzol => None,
            ItemKind::RootedDirt => None,
            ItemKind::CrimsonNylium => None,
            ItemKind::WarpedNylium => None,
            ItemKind::Cobblestone => None,
            ItemKind::OakPlanks => None,
            ItemKind::SprucePlanks => None,
            ItemKind::BirchPlanks => None,
            ItemKind::JunglePlanks => None,
            ItemKind::AcaciaPlanks => None,
            ItemKind::DarkOakPlanks => None,
            ItemKind::CrimsonPlanks => None,
            ItemKind::WarpedPlanks => None,
            ItemKind::OakSapling => None,
            ItemKind::SpruceSapling => None,
            ItemKind::BirchSapling => None,
            ItemKind::JungleSapling => None,
            ItemKind::AcaciaSapling => None,
            ItemKind::DarkOakSapling => None,
            ItemKind::Bedrock => None,
            ItemKind::Sand => None,
            ItemKind::RedSand => None,
            ItemKind::Gravel => None,
            ItemKind::CoalOre => None,
            ItemKind::DeepslateCoalOre => None,
            ItemKind::IronOre => None,
            ItemKind::DeepslateIronOre => None,
            ItemKind::CopperOre => None,
            ItemKind::DeepslateCopperOre => None,
            ItemKind::GoldOre => None,
            ItemKind::DeepslateGoldOre => None,
            ItemKind::RedstoneOre => None,
            ItemKind::DeepslateRedstoneOre => None,
            ItemKind::EmeraldOre => None,
            ItemKind::DeepslateEmeraldOre => None,
            ItemKind::LapisOre => None,
            ItemKind::DeepslateLapisOre => None,
            ItemKind::DiamondOre => None,
            ItemKind::DeepslateDiamondOre => None,
            ItemKind::NetherGoldOre => None,
            ItemKind::NetherQuartzOre => None,
            ItemKind::AncientDebris => None,
            ItemKind::CoalBlock => None,
            ItemKind::RawIronBlock => None,
            ItemKind::RawCopperBlock => None,
            ItemKind::RawGoldBlock => None,
            ItemKind::AmethystBlock => None,
            ItemKind::BuddingAmethyst => None,
            ItemKind::IronBlock => None,
            ItemKind::CopperBlock => None,
            ItemKind::GoldBlock => None,
            ItemKind::DiamondBlock => None,
            ItemKind::NetheriteBlock => None,
            ItemKind::ExposedCopper => None,
            ItemKind::WeatheredCopper => None,
            ItemKind::OxidizedCopper => None,
            ItemKind::CutCopper => None,
            ItemKind::ExposedCutCopper => None,
            ItemKind::WeatheredCutCopper => None,
            ItemKind::OxidizedCutCopper => None,
            ItemKind::CutCopperStairs => None,
            ItemKind::ExposedCutCopperStairs => None,
            ItemKind::WeatheredCutCopperStairs => None,
            ItemKind::OxidizedCutCopperStairs => None,
            ItemKind::CutCopperSlab => None,
            ItemKind::ExposedCutCopperSlab => None,
            ItemKind::WeatheredCutCopperSlab => None,
            ItemKind::OxidizedCutCopperSlab => None,
            ItemKind::WaxedCopperBlock => None,
            ItemKind::WaxedExposedCopper => None,
            ItemKind::WaxedWeatheredCopper => None,
            ItemKind::WaxedOxidizedCopper => None,
            ItemKind::WaxedCutCopper => None,
            ItemKind::WaxedExposedCutCopper => None,
            ItemKind::WaxedWeatheredCutCopper => None,
            ItemKind::WaxedOxidizedCutCopper => None,
            ItemKind::WaxedCutCopperStairs => None,
            ItemKind::WaxedExposedCutCopperStairs => None,
            ItemKind::WaxedWeatheredCutCopperStairs => None,
            ItemKind::WaxedOxidizedCutCopperStairs => None,
            ItemKind::WaxedCutCopperSlab => None,
            ItemKind::WaxedExposedCutCopperSlab => None,
            ItemKind::WaxedWeatheredCutCopperSlab => None,
            ItemKind::WaxedOxidizedCutCopperSlab => None,
            ItemKind::OakLog => None,
            ItemKind::SpruceLog => None,
            ItemKind::BirchLog => None,
            ItemKind::JungleLog => None,
            ItemKind::AcaciaLog => None,
            ItemKind::DarkOakLog => None,
            ItemKind::CrimsonStem => None,
            ItemKind::WarpedStem => None,
            ItemKind::StrippedOakLog => None,
            ItemKind::StrippedSpruceLog => None,
            ItemKind::StrippedBirchLog => None,
            ItemKind::StrippedJungleLog => None,
            ItemKind::StrippedAcaciaLog => None,
            ItemKind::StrippedDarkOakLog => None,
            ItemKind::StrippedCrimsonStem => None,
            ItemKind::StrippedWarpedStem => None,
            ItemKind::StrippedOakWood => None,
            ItemKind::StrippedSpruceWood => None,
            ItemKind::StrippedBirchWood => None,
            ItemKind::StrippedJungleWood => None,
            ItemKind::StrippedAcaciaWood => None,
            ItemKind::StrippedDarkOakWood => None,
            ItemKind::StrippedCrimsonHyphae => None,
            ItemKind::StrippedWarpedHyphae => None,
            ItemKind::OakWood => None,
            ItemKind::SpruceWood => None,
            ItemKind::BirchWood => None,
            ItemKind::JungleWood => None,
            ItemKind::AcaciaWood => None,
            ItemKind::DarkOakWood => None,
            ItemKind::CrimsonHyphae => None,
            ItemKind::WarpedHyphae => None,
            ItemKind::OakLeaves => None,
            ItemKind::SpruceLeaves => None,
            ItemKind::BirchLeaves => None,
            ItemKind::JungleLeaves => None,
            ItemKind::AcaciaLeaves => None,
            ItemKind::DarkOakLeaves => None,
            ItemKind::AzaleaLeaves => None,
            ItemKind::FloweringAzaleaLeaves => None,
            ItemKind::Sponge => None,
            ItemKind::WetSponge => None,
            ItemKind::Glass => None,
            ItemKind::TintedGlass => None,
            ItemKind::LapisBlock => None,
            ItemKind::Sandstone => None,
            ItemKind::ChiseledSandstone => None,
            ItemKind::CutSandstone => None,
            ItemKind::Cobweb => None,
            ItemKind::Grass => None,
            ItemKind::Fern => None,
            ItemKind::Azalea => None,
            ItemKind::FloweringAzalea => None,
            ItemKind::DeadBush => None,
            ItemKind::Seagrass => None,
            ItemKind::SeaPickle => None,
            ItemKind::WhiteWool => None,
            ItemKind::OrangeWool => None,
            ItemKind::MagentaWool => None,
            ItemKind::LightBlueWool => None,
            ItemKind::YellowWool => None,
            ItemKind::LimeWool => None,
            ItemKind::PinkWool => None,
            ItemKind::GrayWool => None,
            ItemKind::LightGrayWool => None,
            ItemKind::CyanWool => None,
            ItemKind::PurpleWool => None,
            ItemKind::BlueWool => None,
            ItemKind::BrownWool => None,
            ItemKind::GreenWool => None,
            ItemKind::RedWool => None,
            ItemKind::BlackWool => None,
            ItemKind::Dandelion => None,
            ItemKind::Poppy => None,
            ItemKind::BlueOrchid => None,
            ItemKind::Allium => None,
            ItemKind::AzureBluet => None,
            ItemKind::RedTulip => None,
            ItemKind::OrangeTulip => None,
            ItemKind::WhiteTulip => None,
            ItemKind::PinkTulip => None,
            ItemKind::OxeyeDaisy => None,
            ItemKind::Cornflower => None,
            ItemKind::LilyOfTheValley => None,
            ItemKind::WitherRose => None,
            ItemKind::SporeBlossom => None,
            ItemKind::BrownMushroom => None,
            ItemKind::RedMushroom => None,
            ItemKind::CrimsonFungus => None,
            ItemKind::WarpedFungus => None,
            ItemKind::CrimsonRoots => None,
            ItemKind::WarpedRoots => None,
            ItemKind::NetherSprouts => None,
            ItemKind::WeepingVines => None,
            ItemKind::TwistingVines => None,
            ItemKind::SugarCane => None,
            ItemKind::Kelp => None,
            ItemKind::MossCarpet => None,
            ItemKind::MossBlock => None,
            ItemKind::HangingRoots => None,
            ItemKind::BigDripleaf => None,
            ItemKind::SmallDripleaf => None,
            ItemKind::Bamboo => None,
            ItemKind::OakSlab => None,
            ItemKind::SpruceSlab => None,
            ItemKind::BirchSlab => None,
            ItemKind::JungleSlab => None,
            ItemKind::AcaciaSlab => None,
            ItemKind::DarkOakSlab => None,
            ItemKind::CrimsonSlab => None,
            ItemKind::WarpedSlab => None,
            ItemKind::StoneSlab => None,
            ItemKind::SmoothStoneSlab => None,
            ItemKind::SandstoneSlab => None,
            ItemKind::CutSandstoneSlab => None,
            ItemKind::PetrifiedOakSlab => None,
            ItemKind::CobblestoneSlab => None,
            ItemKind::BrickSlab => None,
            ItemKind::StoneBrickSlab => None,
            ItemKind::NetherBrickSlab => None,
            ItemKind::QuartzSlab => None,
            ItemKind::RedSandstoneSlab => None,
            ItemKind::CutRedSandstoneSlab => None,
            ItemKind::PurpurSlab => None,
            ItemKind::PrismarineSlab => None,
            ItemKind::PrismarineBrickSlab => None,
            ItemKind::DarkPrismarineSlab => None,
            ItemKind::SmoothQuartz => None,
            ItemKind::SmoothRedSandstone => None,
            ItemKind::SmoothSandstone => None,
            ItemKind::SmoothStone => None,
            ItemKind::Bricks => None,
            ItemKind::Bookshelf => None,
            ItemKind::MossyCobblestone => None,
            ItemKind::Obsidian => None,
            ItemKind::Torch => None,
            ItemKind::EndRod => None,
            ItemKind::ChorusPlant => None,
            ItemKind::ChorusFlower => None,
            ItemKind::PurpurBlock => None,
            ItemKind::PurpurPillar => None,
            ItemKind::PurpurStairs => None,
            ItemKind::Spawner => None,
            ItemKind::OakStairs => None,
            ItemKind::Chest => None,
            ItemKind::CraftingTable => None,
            ItemKind::Farmland => None,
            ItemKind::Furnace => None,
            ItemKind::Ladder => None,
            ItemKind::CobblestoneStairs => None,
            ItemKind::Snow => None,
            ItemKind::Ice => None,
            ItemKind::SnowBlock => None,
            ItemKind::Cactus => None,
            ItemKind::Clay => None,
            ItemKind::Jukebox => None,
            ItemKind::OakFence => None,
            ItemKind::SpruceFence => None,
            ItemKind::BirchFence => None,
            ItemKind::JungleFence => None,
            ItemKind::AcaciaFence => None,
            ItemKind::DarkOakFence => None,
            ItemKind::CrimsonFence => None,
            ItemKind::WarpedFence => None,
            ItemKind::Pumpkin => None,
            ItemKind::CarvedPumpkin => None,
            ItemKind::JackOLantern => None,
            ItemKind::Netherrack => None,
            ItemKind::SoulSand => None,
            ItemKind::SoulSoil => None,
            ItemKind::Basalt => None,
            ItemKind::PolishedBasalt => None,
            ItemKind::SmoothBasalt => None,
            ItemKind::SoulTorch => None,
            ItemKind::Glowstone => None,
            ItemKind::InfestedStone => None,
            ItemKind::InfestedCobblestone => None,
            ItemKind::InfestedStoneBricks => None,
            ItemKind::InfestedMossyStoneBricks => None,
            ItemKind::InfestedCrackedStoneBricks => None,
            ItemKind::InfestedChiseledStoneBricks => None,
            ItemKind::InfestedDeepslate => None,
            ItemKind::StoneBricks => None,
            ItemKind::MossyStoneBricks => None,
            ItemKind::CrackedStoneBricks => None,
            ItemKind::ChiseledStoneBricks => None,
            ItemKind::DeepslateBricks => None,
            ItemKind::CrackedDeepslateBricks => None,
            ItemKind::DeepslateTiles => None,
            ItemKind::CrackedDeepslateTiles => None,
            ItemKind::ChiseledDeepslate => None,
            ItemKind::BrownMushroomBlock => None,
            ItemKind::RedMushroomBlock => None,
            ItemKind::MushroomStem => None,
            ItemKind::IronBars => None,
            ItemKind::Chain => None,
            ItemKind::GlassPane => None,
            ItemKind::Melon => None,
            ItemKind::Vine => None,
            ItemKind::GlowLichen => None,
            ItemKind::BrickStairs => None,
            ItemKind::StoneBrickStairs => None,
            ItemKind::Mycelium => None,
            ItemKind::LilyPad => None,
            ItemKind::NetherBricks => None,
            ItemKind::CrackedNetherBricks => None,
            ItemKind::ChiseledNetherBricks => None,
            ItemKind::NetherBrickFence => None,
            ItemKind::NetherBrickStairs => None,
            ItemKind::EnchantingTable => None,
            ItemKind::EndPortalFrame => None,
            ItemKind::EndStone => None,
            ItemKind::EndStoneBricks => None,
            ItemKind::DragonEgg => None,
            ItemKind::SandstoneStairs => None,
            ItemKind::EnderChest => None,
            ItemKind::EmeraldBlock => None,
            ItemKind::SpruceStairs => None,
            ItemKind::BirchStairs => None,
            ItemKind::JungleStairs => None,
            ItemKind::CrimsonStairs => None,
            ItemKind::WarpedStairs => None,
            ItemKind::CommandBlock => None,
            ItemKind::Beacon => None,
            ItemKind::CobblestoneWall => None,
            ItemKind::MossyCobblestoneWall => None,
            ItemKind::BrickWall => None,
            ItemKind::PrismarineWall => None,
            ItemKind::RedSandstoneWall => None,
            ItemKind::MossyStoneBrickWall => None,
            ItemKind::GraniteWall => None,
            ItemKind::StoneBrickWall => None,
            ItemKind::NetherBrickWall => None,
            ItemKind::AndesiteWall => None,
            ItemKind::RedNetherBrickWall => None,
            ItemKind::SandstoneWall => None,
            ItemKind::EndStoneBrickWall => None,
            ItemKind::DioriteWall => None,
            ItemKind::BlackstoneWall => None,
            ItemKind::PolishedBlackstoneWall => None,
            ItemKind::PolishedBlackstoneBrickWall => None,
            ItemKind::CobbledDeepslateWall => None,
            ItemKind::PolishedDeepslateWall => None,
            ItemKind::DeepslateBrickWall => None,
            ItemKind::DeepslateTileWall => None,
            ItemKind::Anvil => None,
            ItemKind::ChippedAnvil => None,
            ItemKind::DamagedAnvil => None,
            ItemKind::ChiseledQuartzBlock => None,
            ItemKind::QuartzBlock => None,
            ItemKind::QuartzBricks => None,
            ItemKind::QuartzPillar => None,
            ItemKind::QuartzStairs => None,
            ItemKind::WhiteTerracotta => None,
            ItemKind::OrangeTerracotta => None,
            ItemKind::MagentaTerracotta => None,
            ItemKind::LightBlueTerracotta => None,
            ItemKind::YellowTerracotta => None,
            ItemKind::LimeTerracotta => None,
            ItemKind::PinkTerracotta => None,
            ItemKind::GrayTerracotta => None,
            ItemKind::LightGrayTerracotta => None,
            ItemKind::CyanTerracotta => None,
            ItemKind::PurpleTerracotta => None,
            ItemKind::BlueTerracotta => None,
            ItemKind::BrownTerracotta => None,
            ItemKind::GreenTerracotta => None,
            ItemKind::RedTerracotta => None,
            ItemKind::BlackTerracotta => None,
            ItemKind::Barrier => None,
            ItemKind::Light => None,
            ItemKind::HayBlock => None,
            ItemKind::WhiteCarpet => None,
            ItemKind::OrangeCarpet => None,
            ItemKind::MagentaCarpet => None,
            ItemKind::LightBlueCarpet => None,
            ItemKind::YellowCarpet => None,
            ItemKind::LimeCarpet => None,
            ItemKind::PinkCarpet => None,
            ItemKind::GrayCarpet => None,
            ItemKind::LightGrayCarpet => None,
            ItemKind::CyanCarpet => None,
            ItemKind::PurpleCarpet => None,
            ItemKind::BlueCarpet => None,
            ItemKind::BrownCarpet => None,
            ItemKind::GreenCarpet => None,
            ItemKind::RedCarpet => None,
            ItemKind::BlackCarpet => None,
            ItemKind::Terracotta => None,
            ItemKind::PackedIce => None,
            ItemKind::AcaciaStairs => None,
            ItemKind::DarkOakStairs => None,
            ItemKind::DirtPath => None,
            ItemKind::Sunflower => None,
            ItemKind::Lilac => None,
            ItemKind::RoseBush => None,
            ItemKind::Peony => None,
            ItemKind::TallGrass => None,
            ItemKind::LargeFern => None,
            ItemKind::WhiteStainedGlass => None,
            ItemKind::OrangeStainedGlass => None,
            ItemKind::MagentaStainedGlass => None,
            ItemKind::LightBlueStainedGlass => None,
            ItemKind::YellowStainedGlass => None,
            ItemKind::LimeStainedGlass => None,
            ItemKind::PinkStainedGlass => None,
            ItemKind::GrayStainedGlass => None,
            ItemKind::LightGrayStainedGlass => None,
            ItemKind::CyanStainedGlass => None,
            ItemKind::PurpleStainedGlass => None,
            ItemKind::BlueStainedGlass => None,
            ItemKind::BrownStainedGlass => None,
            ItemKind::GreenStainedGlass => None,
            ItemKind::RedStainedGlass => None,
            ItemKind::BlackStainedGlass => None,
            ItemKind::WhiteStainedGlassPane => None,
            ItemKind::OrangeStainedGlassPane => None,
            ItemKind::MagentaStainedGlassPane => None,
            ItemKind::LightBlueStainedGlassPane => None,
            ItemKind::YellowStainedGlassPane => None,
            ItemKind::LimeStainedGlassPane => None,
            ItemKind::PinkStainedGlassPane => None,
            ItemKind::GrayStainedGlassPane => None,
            ItemKind::LightGrayStainedGlassPane => None,
            ItemKind::CyanStainedGlassPane => None,
            ItemKind::PurpleStainedGlassPane => None,
            ItemKind::BlueStainedGlassPane => None,
            ItemKind::BrownStainedGlassPane => None,
            ItemKind::GreenStainedGlassPane => None,
            ItemKind::RedStainedGlassPane => None,
            ItemKind::BlackStainedGlassPane => None,
            ItemKind::Prismarine => None,
            ItemKind::PrismarineBricks => None,
            ItemKind::DarkPrismarine => None,
            ItemKind::PrismarineStairs => None,
            ItemKind::PrismarineBrickStairs => None,
            ItemKind::DarkPrismarineStairs => None,
            ItemKind::SeaLantern => None,
            ItemKind::RedSandstone => None,
            ItemKind::ChiseledRedSandstone => None,
            ItemKind::CutRedSandstone => None,
            ItemKind::RedSandstoneStairs => None,
            ItemKind::RepeatingCommandBlock => None,
            ItemKind::ChainCommandBlock => None,
            ItemKind::MagmaBlock => None,
            ItemKind::NetherWartBlock => None,
            ItemKind::WarpedWartBlock => None,
            ItemKind::RedNetherBricks => None,
            ItemKind::BoneBlock => None,
            ItemKind::StructureVoid => None,
            ItemKind::ShulkerBox => None,
            ItemKind::WhiteShulkerBox => None,
            ItemKind::OrangeShulkerBox => None,
            ItemKind::MagentaShulkerBox => None,
            ItemKind::LightBlueShulkerBox => None,
            ItemKind::YellowShulkerBox => None,
            ItemKind::LimeShulkerBox => None,
            ItemKind::PinkShulkerBox => None,
            ItemKind::GrayShulkerBox => None,
            ItemKind::LightGrayShulkerBox => None,
            ItemKind::CyanShulkerBox => None,
            ItemKind::PurpleShulkerBox => None,
            ItemKind::BlueShulkerBox => None,
            ItemKind::BrownShulkerBox => None,
            ItemKind::GreenShulkerBox => None,
            ItemKind::RedShulkerBox => None,
            ItemKind::BlackShulkerBox => None,
            ItemKind::WhiteGlazedTerracotta => None,
            ItemKind::OrangeGlazedTerracotta => None,
            ItemKind::MagentaGlazedTerracotta => None,
            ItemKind::LightBlueGlazedTerracotta => None,
            ItemKind::YellowGlazedTerracotta => None,
            ItemKind::LimeGlazedTerracotta => None,
            ItemKind::PinkGlazedTerracotta => None,
            ItemKind::GrayGlazedTerracotta => None,
            ItemKind::LightGrayGlazedTerracotta => None,
            ItemKind::CyanGlazedTerracotta => None,
            ItemKind::PurpleGlazedTerracotta => None,
            ItemKind::BlueGlazedTerracotta => None,
            ItemKind::BrownGlazedTerracotta => None,
            ItemKind::GreenGlazedTerracotta => None,
            ItemKind::RedGlazedTerracotta => None,
            ItemKind::BlackGlazedTerracotta => None,
            ItemKind::WhiteConcrete => None,
            ItemKind::OrangeConcrete => None,
            ItemKind::MagentaConcrete => None,
            ItemKind::LightBlueConcrete => None,
            ItemKind::YellowConcrete => None,
            ItemKind::LimeConcrete => None,
            ItemKind::PinkConcrete => None,
            ItemKind::GrayConcrete => None,
            ItemKind::LightGrayConcrete => None,
            ItemKind::CyanConcrete => None,
            ItemKind::PurpleConcrete => None,
            ItemKind::BlueConcrete => None,
            ItemKind::BrownConcrete => None,
            ItemKind::GreenConcrete => None,
            ItemKind::RedConcrete => None,
            ItemKind::BlackConcrete => None,
            ItemKind::WhiteConcretePowder => None,
            ItemKind::OrangeConcretePowder => None,
            ItemKind::MagentaConcretePowder => None,
            ItemKind::LightBlueConcretePowder => None,
            ItemKind::YellowConcretePowder => None,
            ItemKind::LimeConcretePowder => None,
            ItemKind::PinkConcretePowder => None,
            ItemKind::GrayConcretePowder => None,
            ItemKind::LightGrayConcretePowder => None,
            ItemKind::CyanConcretePowder => None,
            ItemKind::PurpleConcretePowder => None,
            ItemKind::BlueConcretePowder => None,
            ItemKind::BrownConcretePowder => None,
            ItemKind::GreenConcretePowder => None,
            ItemKind::RedConcretePowder => None,
            ItemKind::BlackConcretePowder => None,
            ItemKind::TurtleEgg => None,
            ItemKind::DeadTubeCoralBlock => None,
            ItemKind::DeadBrainCoralBlock => None,
            ItemKind::DeadBubbleCoralBlock => None,
            ItemKind::DeadFireCoralBlock => None,
            ItemKind::DeadHornCoralBlock => None,
            ItemKind::TubeCoralBlock => None,
            ItemKind::BrainCoralBlock => None,
            ItemKind::BubbleCoralBlock => None,
            ItemKind::FireCoralBlock => None,
            ItemKind::HornCoralBlock => None,
            ItemKind::TubeCoral => None,
            ItemKind::BrainCoral => None,
            ItemKind::BubbleCoral => None,
            ItemKind::FireCoral => None,
            ItemKind::HornCoral => None,
            ItemKind::DeadBrainCoral => None,
            ItemKind::DeadBubbleCoral => None,
            ItemKind::DeadFireCoral => None,
            ItemKind::DeadHornCoral => None,
            ItemKind::DeadTubeCoral => None,
            ItemKind::TubeCoralFan => None,
            ItemKind::BrainCoralFan => None,
            ItemKind::BubbleCoralFan => None,
            ItemKind::FireCoralFan => None,
            ItemKind::HornCoralFan => None,
            ItemKind::DeadTubeCoralFan => None,
            ItemKind::DeadBrainCoralFan => None,
            ItemKind::DeadBubbleCoralFan => None,
            ItemKind::DeadFireCoralFan => None,
            ItemKind::DeadHornCoralFan => None,
            ItemKind::BlueIce => None,
            ItemKind::Conduit => None,
            ItemKind::PolishedGraniteStairs => None,
            ItemKind::SmoothRedSandstoneStairs => None,
            ItemKind::MossyStoneBrickStairs => None,
            ItemKind::PolishedDioriteStairs => None,
            ItemKind::MossyCobblestoneStairs => None,
            ItemKind::EndStoneBrickStairs => None,
            ItemKind::StoneStairs => None,
            ItemKind::SmoothSandstoneStairs => None,
            ItemKind::SmoothQuartzStairs => None,
            ItemKind::GraniteStairs => None,
            ItemKind::AndesiteStairs => None,
            ItemKind::RedNetherBrickStairs => None,
            ItemKind::PolishedAndesiteStairs => None,
            ItemKind::DioriteStairs => None,
            ItemKind::CobbledDeepslateStairs => None,
            ItemKind::PolishedDeepslateStairs => None,
            ItemKind::DeepslateBrickStairs => None,
            ItemKind::DeepslateTileStairs => None,
            ItemKind::PolishedGraniteSlab => None,
            ItemKind::SmoothRedSandstoneSlab => None,
            ItemKind::MossyStoneBrickSlab => None,
            ItemKind::PolishedDioriteSlab => None,
            ItemKind::MossyCobblestoneSlab => None,
            ItemKind::EndStoneBrickSlab => None,
            ItemKind::SmoothSandstoneSlab => None,
            ItemKind::SmoothQuartzSlab => None,
            ItemKind::GraniteSlab => None,
            ItemKind::AndesiteSlab => None,
            ItemKind::RedNetherBrickSlab => None,
            ItemKind::PolishedAndesiteSlab => None,
            ItemKind::DioriteSlab => None,
            ItemKind::CobbledDeepslateSlab => None,
            ItemKind::PolishedDeepslateSlab => None,
            ItemKind::DeepslateBrickSlab => None,
            ItemKind::DeepslateTileSlab => None,
            ItemKind::Scaffolding => None,
            ItemKind::Redstone => None,
            ItemKind::RedstoneTorch => None,
            ItemKind::RedstoneBlock => None,
            ItemKind::Repeater => None,
            ItemKind::Comparator => None,
            ItemKind::Piston => None,
            ItemKind::StickyPiston => None,
            ItemKind::SlimeBlock => None,
            ItemKind::HoneyBlock => None,
            ItemKind::Observer => None,
            ItemKind::Hopper => None,
            ItemKind::Dispenser => None,
            ItemKind::Dropper => None,
            ItemKind::Lectern => None,
            ItemKind::Target => None,
            ItemKind::Lever => None,
            ItemKind::LightningRod => None,
            ItemKind::DaylightDetector => None,
            ItemKind::SculkSensor => None,
            ItemKind::TripwireHook => None,
            ItemKind::TrappedChest => None,
            ItemKind::Tnt => None,
            ItemKind::RedstoneLamp => None,
            ItemKind::NoteBlock => None,
            ItemKind::StoneButton => None,
            ItemKind::PolishedBlackstoneButton => None,
            ItemKind::OakButton => None,
            ItemKind::SpruceButton => None,
            ItemKind::BirchButton => None,
            ItemKind::JungleButton => None,
            ItemKind::AcaciaButton => None,
            ItemKind::DarkOakButton => None,
            ItemKind::CrimsonButton => None,
            ItemKind::WarpedButton => None,
            ItemKind::StonePressurePlate => None,
            ItemKind::PolishedBlackstonePressurePlate => None,
            ItemKind::LightWeightedPressurePlate => None,
            ItemKind::HeavyWeightedPressurePlate => None,
            ItemKind::OakPressurePlate => None,
            ItemKind::SprucePressurePlate => None,
            ItemKind::BirchPressurePlate => None,
            ItemKind::JunglePressurePlate => None,
            ItemKind::AcaciaPressurePlate => None,
            ItemKind::DarkOakPressurePlate => None,
            ItemKind::CrimsonPressurePlate => None,
            ItemKind::WarpedPressurePlate => None,
            ItemKind::IronDoor => None,
            ItemKind::OakDoor => None,
            ItemKind::SpruceDoor => None,
            ItemKind::BirchDoor => None,
            ItemKind::JungleDoor => None,
            ItemKind::AcaciaDoor => None,
            ItemKind::DarkOakDoor => None,
            ItemKind::CrimsonDoor => None,
            ItemKind::WarpedDoor => None,
            ItemKind::IronTrapdoor => None,
            ItemKind::OakTrapdoor => None,
            ItemKind::SpruceTrapdoor => None,
            ItemKind::BirchTrapdoor => None,
            ItemKind::JungleTrapdoor => None,
            ItemKind::AcaciaTrapdoor => None,
            ItemKind::DarkOakTrapdoor => None,
            ItemKind::CrimsonTrapdoor => None,
            ItemKind::WarpedTrapdoor => None,
            ItemKind::OakFenceGate => None,
            ItemKind::SpruceFenceGate => None,
            ItemKind::BirchFenceGate => None,
            ItemKind::JungleFenceGate => None,
            ItemKind::AcaciaFenceGate => None,
            ItemKind::DarkOakFenceGate => None,
            ItemKind::CrimsonFenceGate => None,
            ItemKind::WarpedFenceGate => None,
            ItemKind::PoweredRail => None,
            ItemKind::DetectorRail => None,
            ItemKind::Rail => None,
            ItemKind::ActivatorRail => None,
            ItemKind::Saddle => None,
            ItemKind::Minecart => None,
            ItemKind::ChestMinecart => None,
            ItemKind::FurnaceMinecart => None,
            ItemKind::TntMinecart => None,
            ItemKind::HopperMinecart => None,
            ItemKind::CarrotOnAStick => Some(25u32),
            ItemKind::WarpedFungusOnAStick => Some(100u32),
            ItemKind::Elytra => Some(432u32),
            ItemKind::OakBoat => None,
            ItemKind::SpruceBoat => None,
            ItemKind::BirchBoat => None,
            ItemKind::JungleBoat => None,
            ItemKind::AcaciaBoat => None,
            ItemKind::DarkOakBoat => None,
            ItemKind::StructureBlock => None,
            ItemKind::Jigsaw => None,
            ItemKind::TurtleHelmet => Some(275u32),
            ItemKind::Scute => None,
            ItemKind::FlintAndSteel => Some(64u32),
            ItemKind::Apple => None,
            ItemKind::Bow => Some(384u32),
            ItemKind::Arrow => None,
            ItemKind::Coal => None,
            ItemKind::Charcoal => None,
            ItemKind::Diamond => None,
            ItemKind::Emerald => None,
            ItemKind::LapisLazuli => None,
            ItemKind::Quartz => None,
            ItemKind::AmethystShard => None,
            ItemKind::RawIron => None,
            ItemKind::IronIngot => None,
            ItemKind::RawCopper => None,
            ItemKind::CopperIngot => None,
            ItemKind::RawGold => None,
            ItemKind::GoldIngot => None,
            ItemKind::NetheriteIngot => None,
            ItemKind::NetheriteScrap => None,
            ItemKind::WoodenSword => Some(59u32),
            ItemKind::WoodenShovel => Some(59u32),
            ItemKind::WoodenPickaxe => Some(59u32),
            ItemKind::WoodenAxe => Some(59u32),
            ItemKind::WoodenHoe => Some(59u32),
            ItemKind::StoneSword => Some(131u32),
            ItemKind::StoneShovel => Some(131u32),
            ItemKind::StonePickaxe => Some(131u32),
            ItemKind::StoneAxe => Some(131u32),
            ItemKind::StoneHoe => Some(131u32),
            ItemKind::GoldenSword => Some(32u32),
            ItemKind::GoldenShovel => Some(32u32),
            ItemKind::GoldenPickaxe => Some(32u32),
            ItemKind::GoldenAxe => Some(32u32),
            ItemKind::GoldenHoe => Some(32u32),
            ItemKind::IronSword => Some(250u32),
            ItemKind::IronShovel => Some(250u32),
            ItemKind::IronPickaxe => Some(250u32),
            ItemKind::IronAxe => Some(250u32),
            ItemKind::IronHoe => Some(250u32),
            ItemKind::DiamondSword => Some(1561u32),
            ItemKind::DiamondShovel => Some(1561u32),
            ItemKind::DiamondPickaxe => Some(1561u32),
            ItemKind::DiamondAxe => Some(1561u32),
            ItemKind::DiamondHoe => Some(1561u32),
            ItemKind::NetheriteSword => Some(2031u32),
            ItemKind::NetheriteShovel => Some(2031u32),
            ItemKind::NetheritePickaxe => Some(2031u32),
            ItemKind::NetheriteAxe => Some(2031u32),
            ItemKind::NetheriteHoe => Some(2031u32),
            ItemKind::Stick => None,
            ItemKind::Bowl => None,
            ItemKind::MushroomStew => None,
            ItemKind::String => None,
            ItemKind::Feather => None,
            ItemKind::Gunpowder => None,
            ItemKind::WheatSeeds => None,
            ItemKind::Wheat => None,
            ItemKind::Bread => None,
            ItemKind::LeatherHelmet => Some(55u32),
            ItemKind::LeatherChestplate => Some(80u32),
            ItemKind::LeatherLeggings => Some(75u32),
            ItemKind::LeatherBoots => Some(65u32),
            ItemKind::ChainmailHelmet => Some(165u32),
            ItemKind::ChainmailChestplate => Some(240u32),
            ItemKind::ChainmailLeggings => Some(225u32),
            ItemKind::ChainmailBoots => Some(195u32),
            ItemKind::IronHelmet => Some(165u32),
            ItemKind::IronChestplate => Some(240u32),
            ItemKind::IronLeggings => Some(225u32),
            ItemKind::IronBoots => Some(195u32),
            ItemKind::DiamondHelmet => Some(363u32),
            ItemKind::DiamondChestplate => Some(528u32),
            ItemKind::DiamondLeggings => Some(495u32),
            ItemKind::DiamondBoots => Some(429u32),
            ItemKind::GoldenHelmet => Some(77u32),
            ItemKind::GoldenChestplate => Some(112u32),
            ItemKind::GoldenLeggings => Some(105u32),
            ItemKind::GoldenBoots => Some(91u32),
            ItemKind::NetheriteHelmet => Some(407u32),
            ItemKind::NetheriteChestplate => Some(592u32),
            ItemKind::NetheriteLeggings => Some(555u32),
            ItemKind::NetheriteBoots => Some(481u32),
            ItemKind::Flint => None,
            ItemKind::Porkchop => None,
            ItemKind::CookedPorkchop => None,
            ItemKind::Painting => None,
            ItemKind::GoldenApple => None,
            ItemKind::EnchantedGoldenApple => None,
            ItemKind::OakSign => None,
            ItemKind::SpruceSign => None,
            ItemKind::BirchSign => None,
            ItemKind::JungleSign => None,
            ItemKind::AcaciaSign => None,
            ItemKind::DarkOakSign => None,
            ItemKind::CrimsonSign => None,
            ItemKind::WarpedSign => None,
            ItemKind::Bucket => None,
            ItemKind::WaterBucket => None,
            ItemKind::LavaBucket => None,
            ItemKind::PowderSnowBucket => None,
            ItemKind::Snowball => None,
            ItemKind::Leather => None,
            ItemKind::MilkBucket => None,
            ItemKind::PufferfishBucket => None,
            ItemKind::SalmonBucket => None,
            ItemKind::CodBucket => None,
            ItemKind::TropicalFishBucket => None,
            ItemKind::AxolotlBucket => None,
            ItemKind::Brick => None,
            ItemKind::ClayBall => None,
            ItemKind::DriedKelpBlock => None,
            ItemKind::Paper => None,
            ItemKind::Book => None,
            ItemKind::SlimeBall => None,
            ItemKind::Egg => None,
            ItemKind::Compass => None,
            ItemKind::Bundle => None,
            ItemKind::FishingRod => Some(64u32),
            ItemKind::Clock => None,
            ItemKind::Spyglass => None,
            ItemKind::GlowstoneDust => None,
            ItemKind::Cod => None,
            ItemKind::Salmon => None,
            ItemKind::TropicalFish => None,
            ItemKind::Pufferfish => None,
            ItemKind::CookedCod => None,
            ItemKind::CookedSalmon => None,
            ItemKind::InkSac => None,
            ItemKind::GlowInkSac => None,
            ItemKind::CocoaBeans => None,
            ItemKind::WhiteDye => None,
            ItemKind::OrangeDye => None,
            ItemKind::MagentaDye => None,
            ItemKind::LightBlueDye => None,
            ItemKind::YellowDye => None,
            ItemKind::LimeDye => None,
            ItemKind::PinkDye => None,
            ItemKind::GrayDye => None,
            ItemKind::LightGrayDye => None,
            ItemKind::CyanDye => None,
            ItemKind::PurpleDye => None,
            ItemKind::BlueDye => None,
            ItemKind::BrownDye => None,
            ItemKind::GreenDye => None,
            ItemKind::RedDye => None,
            ItemKind::BlackDye => None,
            ItemKind::BoneMeal => None,
            ItemKind::Bone => None,
            ItemKind::Sugar => None,
            ItemKind::Cake => None,
            ItemKind::WhiteBed => None,
            ItemKind::OrangeBed => None,
            ItemKind::MagentaBed => None,
            ItemKind::LightBlueBed => None,
            ItemKind::YellowBed => None,
            ItemKind::LimeBed => None,
            ItemKind::PinkBed => None,
            ItemKind::GrayBed => None,
            ItemKind::LightGrayBed => None,
            ItemKind::CyanBed => None,
            ItemKind::PurpleBed => None,
            ItemKind::BlueBed => None,
            ItemKind::BrownBed => None,
            ItemKind::GreenBed => None,
            ItemKind::RedBed => None,
            ItemKind::BlackBed => None,
            ItemKind::Cookie => None,
            ItemKind::FilledMap => None,
            ItemKind::Shears => Some(238u32),
            ItemKind::MelonSlice => None,
            ItemKind::DriedKelp => None,
            ItemKind::PumpkinSeeds => None,
            ItemKind::MelonSeeds => None,
            ItemKind::Beef => None,
            ItemKind::CookedBeef => None,
            ItemKind::Chicken => None,
            ItemKind::CookedChicken => None,
            ItemKind::RottenFlesh => None,
            ItemKind::EnderPearl => None,
            ItemKind::BlazeRod => None,
            ItemKind::GhastTear => None,
            ItemKind::GoldNugget => None,
            ItemKind::NetherWart => None,
            ItemKind::Potion => None,
            ItemKind::GlassBottle => None,
            ItemKind::SpiderEye => None,
            ItemKind::FermentedSpiderEye => None,
            ItemKind::BlazePowder => None,
            ItemKind::MagmaCream => None,
            ItemKind::BrewingStand => None,
            ItemKind::Cauldron => None,
            ItemKind::EnderEye => None,
            ItemKind::GlisteringMelonSlice => None,
            ItemKind::AxolotlSpawnEgg => None,
            ItemKind::BatSpawnEgg => None,
            ItemKind::BeeSpawnEgg => None,
            ItemKind::BlazeSpawnEgg => None,
            ItemKind::CatSpawnEgg => None,
            ItemKind::CaveSpiderSpawnEgg => None,
            ItemKind::ChickenSpawnEgg => None,
            ItemKind::CodSpawnEgg => None,
            ItemKind::CowSpawnEgg => None,
            ItemKind::CreeperSpawnEgg => None,
            ItemKind::DolphinSpawnEgg => None,
            ItemKind::DonkeySpawnEgg => None,
            ItemKind::DrownedSpawnEgg => None,
            ItemKind::ElderGuardianSpawnEgg => None,
            ItemKind::EndermanSpawnEgg => None,
            ItemKind::EndermiteSpawnEgg => None,
            ItemKind::EvokerSpawnEgg => None,
            ItemKind::FoxSpawnEgg => None,
            ItemKind::GhastSpawnEgg => None,
            ItemKind::GlowSquidSpawnEgg => None,
            ItemKind::GoatSpawnEgg => None,
            ItemKind::GuardianSpawnEgg => None,
            ItemKind::HoglinSpawnEgg => None,
            ItemKind::HorseSpawnEgg => None,
            ItemKind::HuskSpawnEgg => None,
            ItemKind::LlamaSpawnEgg => None,
            ItemKind::MagmaCubeSpawnEgg => None,
            ItemKind::MooshroomSpawnEgg => None,
            ItemKind::MuleSpawnEgg => None,
            ItemKind::OcelotSpawnEgg => None,
            ItemKind::PandaSpawnEgg => None,
            ItemKind::ParrotSpawnEgg => None,
            ItemKind::PhantomSpawnEgg => None,
            ItemKind::PigSpawnEgg => None,
            ItemKind::PiglinSpawnEgg => None,
            ItemKind::PiglinBruteSpawnEgg => None,
            ItemKind::PillagerSpawnEgg => None,
            ItemKind::PolarBearSpawnEgg => None,
            ItemKind::PufferfishSpawnEgg => None,
            ItemKind::RabbitSpawnEgg => None,
            ItemKind::RavagerSpawnEgg => None,
            ItemKind::SalmonSpawnEgg => None,
            ItemKind::SheepSpawnEgg => None,
            ItemKind::ShulkerSpawnEgg => None,
            ItemKind::SilverfishSpawnEgg => None,
            ItemKind::SkeletonSpawnEgg => None,
            ItemKind::SkeletonHorseSpawnEgg => None,
            ItemKind::SlimeSpawnEgg => None,
            ItemKind::SpiderSpawnEgg => None,
            ItemKind::SquidSpawnEgg => None,
            ItemKind::StraySpawnEgg => None,
            ItemKind::StriderSpawnEgg => None,
            ItemKind::TraderLlamaSpawnEgg => None,
            ItemKind::TropicalFishSpawnEgg => None,
            ItemKind::TurtleSpawnEgg => None,
            ItemKind::VexSpawnEgg => None,
            ItemKind::VillagerSpawnEgg => None,
            ItemKind::VindicatorSpawnEgg => None,
            ItemKind::WanderingTraderSpawnEgg => None,
            ItemKind::WitchSpawnEgg => None,
            ItemKind::WitherSkeletonSpawnEgg => None,
            ItemKind::WolfSpawnEgg => None,
            ItemKind::ZoglinSpawnEgg => None,
            ItemKind::ZombieSpawnEgg => None,
            ItemKind::ZombieHorseSpawnEgg => None,
            ItemKind::ZombieVillagerSpawnEgg => None,
            ItemKind::ZombifiedPiglinSpawnEgg => None,
            ItemKind::ExperienceBottle => None,
            ItemKind::FireCharge => None,
            ItemKind::WritableBook => None,
            ItemKind::WrittenBook => None,
            ItemKind::ItemFrame => None,
            ItemKind::GlowItemFrame => None,
            ItemKind::FlowerPot => None,
            ItemKind::Carrot => None,
            ItemKind::Potato => None,
            ItemKind::BakedPotato => None,
            ItemKind::PoisonousPotato => None,
            ItemKind::Map => None,
            ItemKind::GoldenCarrot => None,
            ItemKind::SkeletonSkull => None,
            ItemKind::WitherSkeletonSkull => None,
            ItemKind::PlayerHead => None,
            ItemKind::ZombieHead => None,
            ItemKind::CreeperHead => None,
            ItemKind::DragonHead => None,
            ItemKind::NetherStar => None,
            ItemKind::PumpkinPie => None,
            ItemKind::FireworkRocket => None,
            ItemKind::FireworkStar => None,
            ItemKind::EnchantedBook => None,
            ItemKind::NetherBrick => None,
            ItemKind::PrismarineShard => None,
            ItemKind::PrismarineCrystals => None,
            ItemKind::Rabbit => None,
            ItemKind::CookedRabbit => None,
            ItemKind::RabbitStew => None,
            ItemKind::RabbitFoot => None,
            ItemKind::RabbitHide => None,
            ItemKind::ArmorStand => None,
            ItemKind::IronHorseArmor => None,
            ItemKind::GoldenHorseArmor => None,
            ItemKind::DiamondHorseArmor => None,
            ItemKind::LeatherHorseArmor => None,
            ItemKind::Lead => None,
            ItemKind::NameTag => None,
            ItemKind::CommandBlockMinecart => None,
            ItemKind::Mutton => None,
            ItemKind::CookedMutton => None,
            ItemKind::WhiteBanner => None,
            ItemKind::OrangeBanner => None,
            ItemKind::MagentaBanner => None,
            ItemKind::LightBlueBanner => None,
            ItemKind::YellowBanner => None,
            ItemKind::LimeBanner => None,
            ItemKind::PinkBanner => None,
            ItemKind::GrayBanner => None,
            ItemKind::LightGrayBanner => None,
            ItemKind::CyanBanner => None,
            ItemKind::PurpleBanner => None,
            ItemKind::BlueBanner => None,
            ItemKind::BrownBanner => None,
            ItemKind::GreenBanner => None,
            ItemKind::RedBanner => None,
            ItemKind::BlackBanner => None,
            ItemKind::EndCrystal => None,
            ItemKind::ChorusFruit => None,
            ItemKind::PoppedChorusFruit => None,
            ItemKind::Beetroot => None,
            ItemKind::BeetrootSeeds => None,
            ItemKind::BeetrootSoup => None,
            ItemKind::DragonBreath => None,
            ItemKind::SplashPotion => None,
            ItemKind::SpectralArrow => None,
            ItemKind::TippedArrow => None,
            ItemKind::LingeringPotion => None,
            ItemKind::Shield => Some(336u32),
            ItemKind::TotemOfUndying => None,
            ItemKind::ShulkerShell => None,
            ItemKind::IronNugget => None,
            ItemKind::KnowledgeBook => None,
            ItemKind::DebugStick => None,
            ItemKind::MusicDisc13 => None,
            ItemKind::MusicDiscCat => None,
            ItemKind::MusicDiscBlocks => None,
            ItemKind::MusicDiscChirp => None,
            ItemKind::MusicDiscFar => None,
            ItemKind::MusicDiscMall => None,
            ItemKind::MusicDiscMellohi => None,
            ItemKind::MusicDiscStal => None,
            ItemKind::MusicDiscStrad => None,
            ItemKind::MusicDiscWard => None,
            ItemKind::MusicDisc11 => None,
            ItemKind::MusicDiscWait => None,
            ItemKind::MusicDiscOtherside => None,
            ItemKind::MusicDiscPigstep => None,
            ItemKind::Trident => Some(250u32),
            ItemKind::PhantomMembrane => None,
            ItemKind::NautilusShell => None,
            ItemKind::HeartOfTheSea => None,
            ItemKind::Crossbow => Some(326u32),
            ItemKind::SuspiciousStew => None,
            ItemKind::Loom => None,
            ItemKind::FlowerBannerPattern => None,
            ItemKind::CreeperBannerPattern => None,
            ItemKind::SkullBannerPattern => None,
            ItemKind::MojangBannerPattern => None,
            ItemKind::GlobeBannerPattern => None,
            ItemKind::PiglinBannerPattern => None,
            ItemKind::Composter => None,
            ItemKind::Barrel => None,
            ItemKind::Smoker => None,
            ItemKind::BlastFurnace => None,
            ItemKind::CartographyTable => None,
            ItemKind::FletchingTable => None,
            ItemKind::Grindstone => None,
            ItemKind::SmithingTable => None,
            ItemKind::Stonecutter => None,
            ItemKind::Bell => None,
            ItemKind::Lantern => None,
            ItemKind::SoulLantern => None,
            ItemKind::SweetBerries => None,
            ItemKind::GlowBerries => None,
            ItemKind::Campfire => None,
            ItemKind::SoulCampfire => None,
            ItemKind::Shroomlight => None,
            ItemKind::Honeycomb => None,
            ItemKind::BeeNest => None,
            ItemKind::Beehive => None,
            ItemKind::HoneyBottle => None,
            ItemKind::HoneycombBlock => None,
            ItemKind::Lodestone => None,
            ItemKind::CryingObsidian => None,
            ItemKind::Blackstone => None,
            ItemKind::BlackstoneSlab => None,
            ItemKind::BlackstoneStairs => None,
            ItemKind::GildedBlackstone => None,
            ItemKind::PolishedBlackstone => None,
            ItemKind::PolishedBlackstoneSlab => None,
            ItemKind::PolishedBlackstoneStairs => None,
            ItemKind::ChiseledPolishedBlackstone => None,
            ItemKind::PolishedBlackstoneBricks => None,
            ItemKind::PolishedBlackstoneBrickSlab => None,
            ItemKind::PolishedBlackstoneBrickStairs => None,
            ItemKind::CrackedPolishedBlackstoneBricks => None,
            ItemKind::RespawnAnchor => None,
            ItemKind::Candle => None,
            ItemKind::WhiteCandle => None,
            ItemKind::OrangeCandle => None,
            ItemKind::MagentaCandle => None,
            ItemKind::LightBlueCandle => None,
            ItemKind::YellowCandle => None,
            ItemKind::LimeCandle => None,
            ItemKind::PinkCandle => None,
            ItemKind::GrayCandle => None,
            ItemKind::LightGrayCandle => None,
            ItemKind::CyanCandle => None,
            ItemKind::PurpleCandle => None,
            ItemKind::BlueCandle => None,
            ItemKind::BrownCandle => None,
            ItemKind::GreenCandle => None,
            ItemKind::RedCandle => None,
            ItemKind::BlackCandle => None,
            ItemKind::SmallAmethystBud => None,
            ItemKind::MediumAmethystBud => None,
            ItemKind::LargeAmethystBud => None,
            ItemKind::AmethystCluster => None,
            ItemKind::PointedDripstone => None,
        }
    }
}
impl ItemKind {
    #[doc = "Returns the `fixed_with` property of this `ItemKind`."]
    #[inline]
    pub const fn fixed_with(&self) -> Vec<&'static str> {
        match self {
            ItemKind::Stone => {
                vec![]
            }
            ItemKind::Granite => {
                vec![]
            }
            ItemKind::PolishedGranite => {
                vec![]
            }
            ItemKind::Diorite => {
                vec![]
            }
            ItemKind::PolishedDiorite => {
                vec![]
            }
            ItemKind::Andesite => {
                vec![]
            }
            ItemKind::PolishedAndesite => {
                vec![]
            }
            ItemKind::Deepslate => {
                vec![]
            }
            ItemKind::CobbledDeepslate => {
                vec![]
            }
            ItemKind::PolishedDeepslate => {
                vec![]
            }
            ItemKind::Calcite => {
                vec![]
            }
            ItemKind::Tuff => {
                vec![]
            }
            ItemKind::DripstoneBlock => {
                vec![]
            }
            ItemKind::GrassBlock => {
                vec![]
            }
            ItemKind::Dirt => {
                vec![]
            }
            ItemKind::CoarseDirt => {
                vec![]
            }
            ItemKind::Podzol => {
                vec![]
            }
            ItemKind::RootedDirt => {
                vec![]
            }
            ItemKind::CrimsonNylium => {
                vec![]
            }
            ItemKind::WarpedNylium => {
                vec![]
            }
            ItemKind::Cobblestone => {
                vec![]
            }
            ItemKind::OakPlanks => {
                vec![]
            }
            ItemKind::SprucePlanks => {
                vec![]
            }
            ItemKind::BirchPlanks => {
                vec![]
            }
            ItemKind::JunglePlanks => {
                vec![]
            }
            ItemKind::AcaciaPlanks => {
                vec![]
            }
            ItemKind::DarkOakPlanks => {
                vec![]
            }
            ItemKind::CrimsonPlanks => {
                vec![]
            }
            ItemKind::WarpedPlanks => {
                vec![]
            }
            ItemKind::OakSapling => {
                vec![]
            }
            ItemKind::SpruceSapling => {
                vec![]
            }
            ItemKind::BirchSapling => {
                vec![]
            }
            ItemKind::JungleSapling => {
                vec![]
            }
            ItemKind::AcaciaSapling => {
                vec![]
            }
            ItemKind::DarkOakSapling => {
                vec![]
            }
            ItemKind::Bedrock => {
                vec![]
            }
            ItemKind::Sand => {
                vec![]
            }
            ItemKind::RedSand => {
                vec![]
            }
            ItemKind::Gravel => {
                vec![]
            }
            ItemKind::CoalOre => {
                vec![]
            }
            ItemKind::DeepslateCoalOre => {
                vec![]
            }
            ItemKind::IronOre => {
                vec![]
            }
            ItemKind::DeepslateIronOre => {
                vec![]
            }
            ItemKind::CopperOre => {
                vec![]
            }
            ItemKind::DeepslateCopperOre => {
                vec![]
            }
            ItemKind::GoldOre => {
                vec![]
            }
            ItemKind::DeepslateGoldOre => {
                vec![]
            }
            ItemKind::RedstoneOre => {
                vec![]
            }
            ItemKind::DeepslateRedstoneOre => {
                vec![]
            }
            ItemKind::EmeraldOre => {
                vec![]
            }
            ItemKind::DeepslateEmeraldOre => {
                vec![]
            }
            ItemKind::LapisOre => {
                vec![]
            }
            ItemKind::DeepslateLapisOre => {
                vec![]
            }
            ItemKind::DiamondOre => {
                vec![]
            }
            ItemKind::DeepslateDiamondOre => {
                vec![]
            }
            ItemKind::NetherGoldOre => {
                vec![]
            }
            ItemKind::NetherQuartzOre => {
                vec![]
            }
            ItemKind::AncientDebris => {
                vec![]
            }
            ItemKind::CoalBlock => {
                vec![]
            }
            ItemKind::RawIronBlock => {
                vec![]
            }
            ItemKind::RawCopperBlock => {
                vec![]
            }
            ItemKind::RawGoldBlock => {
                vec![]
            }
            ItemKind::AmethystBlock => {
                vec![]
            }
            ItemKind::BuddingAmethyst => {
                vec![]
            }
            ItemKind::IronBlock => {
                vec![]
            }
            ItemKind::CopperBlock => {
                vec![]
            }
            ItemKind::GoldBlock => {
                vec![]
            }
            ItemKind::DiamondBlock => {
                vec![]
            }
            ItemKind::NetheriteBlock => {
                vec![]
            }
            ItemKind::ExposedCopper => {
                vec![]
            }
            ItemKind::WeatheredCopper => {
                vec![]
            }
            ItemKind::OxidizedCopper => {
                vec![]
            }
            ItemKind::CutCopper => {
                vec![]
            }
            ItemKind::ExposedCutCopper => {
                vec![]
            }
            ItemKind::WeatheredCutCopper => {
                vec![]
            }
            ItemKind::OxidizedCutCopper => {
                vec![]
            }
            ItemKind::CutCopperStairs => {
                vec![]
            }
            ItemKind::ExposedCutCopperStairs => {
                vec![]
            }
            ItemKind::WeatheredCutCopperStairs => {
                vec![]
            }
            ItemKind::OxidizedCutCopperStairs => {
                vec![]
            }
            ItemKind::CutCopperSlab => {
                vec![]
            }
            ItemKind::ExposedCutCopperSlab => {
                vec![]
            }
            ItemKind::WeatheredCutCopperSlab => {
                vec![]
            }
            ItemKind::OxidizedCutCopperSlab => {
                vec![]
            }
            ItemKind::WaxedCopperBlock => {
                vec![]
            }
            ItemKind::WaxedExposedCopper => {
                vec![]
            }
            ItemKind::WaxedWeatheredCopper => {
                vec![]
            }
            ItemKind::WaxedOxidizedCopper => {
                vec![]
            }
            ItemKind::WaxedCutCopper => {
                vec![]
            }
            ItemKind::WaxedExposedCutCopper => {
                vec![]
            }
            ItemKind::WaxedWeatheredCutCopper => {
                vec![]
            }
            ItemKind::WaxedOxidizedCutCopper => {
                vec![]
            }
            ItemKind::WaxedCutCopperStairs => {
                vec![]
            }
            ItemKind::WaxedExposedCutCopperStairs => {
                vec![]
            }
            ItemKind::WaxedWeatheredCutCopperStairs => {
                vec![]
            }
            ItemKind::WaxedOxidizedCutCopperStairs => {
                vec![]
            }
            ItemKind::WaxedCutCopperSlab => {
                vec![]
            }
            ItemKind::WaxedExposedCutCopperSlab => {
                vec![]
            }
            ItemKind::WaxedWeatheredCutCopperSlab => {
                vec![]
            }
            ItemKind::WaxedOxidizedCutCopperSlab => {
                vec![]
            }
            ItemKind::OakLog => {
                vec![]
            }
            ItemKind::SpruceLog => {
                vec![]
            }
            ItemKind::BirchLog => {
                vec![]
            }
            ItemKind::JungleLog => {
                vec![]
            }
            ItemKind::AcaciaLog => {
                vec![]
            }
            ItemKind::DarkOakLog => {
                vec![]
            }
            ItemKind::CrimsonStem => {
                vec![]
            }
            ItemKind::WarpedStem => {
                vec![]
            }
            ItemKind::StrippedOakLog => {
                vec![]
            }
            ItemKind::StrippedSpruceLog => {
                vec![]
            }
            ItemKind::StrippedBirchLog => {
                vec![]
            }
            ItemKind::StrippedJungleLog => {
                vec![]
            }
            ItemKind::StrippedAcaciaLog => {
                vec![]
            }
            ItemKind::StrippedDarkOakLog => {
                vec![]
            }
            ItemKind::StrippedCrimsonStem => {
                vec![]
            }
            ItemKind::StrippedWarpedStem => {
                vec![]
            }
            ItemKind::StrippedOakWood => {
                vec![]
            }
            ItemKind::StrippedSpruceWood => {
                vec![]
            }
            ItemKind::StrippedBirchWood => {
                vec![]
            }
            ItemKind::StrippedJungleWood => {
                vec![]
            }
            ItemKind::StrippedAcaciaWood => {
                vec![]
            }
            ItemKind::StrippedDarkOakWood => {
                vec![]
            }
            ItemKind::StrippedCrimsonHyphae => {
                vec![]
            }
            ItemKind::StrippedWarpedHyphae => {
                vec![]
            }
            ItemKind::OakWood => {
                vec![]
            }
            ItemKind::SpruceWood => {
                vec![]
            }
            ItemKind::BirchWood => {
                vec![]
            }
            ItemKind::JungleWood => {
                vec![]
            }
            ItemKind::AcaciaWood => {
                vec![]
            }
            ItemKind::DarkOakWood => {
                vec![]
            }
            ItemKind::CrimsonHyphae => {
                vec![]
            }
            ItemKind::WarpedHyphae => {
                vec![]
            }
            ItemKind::OakLeaves => {
                vec![]
            }
            ItemKind::SpruceLeaves => {
                vec![]
            }
            ItemKind::BirchLeaves => {
                vec![]
            }
            ItemKind::JungleLeaves => {
                vec![]
            }
            ItemKind::AcaciaLeaves => {
                vec![]
            }
            ItemKind::DarkOakLeaves => {
                vec![]
            }
            ItemKind::AzaleaLeaves => {
                vec![]
            }
            ItemKind::FloweringAzaleaLeaves => {
                vec![]
            }
            ItemKind::Sponge => {
                vec![]
            }
            ItemKind::WetSponge => {
                vec![]
            }
            ItemKind::Glass => {
                vec![]
            }
            ItemKind::TintedGlass => {
                vec![]
            }
            ItemKind::LapisBlock => {
                vec![]
            }
            ItemKind::Sandstone => {
                vec![]
            }
            ItemKind::ChiseledSandstone => {
                vec![]
            }
            ItemKind::CutSandstone => {
                vec![]
            }
            ItemKind::Cobweb => {
                vec![]
            }
            ItemKind::Grass => {
                vec![]
            }
            ItemKind::Fern => {
                vec![]
            }
            ItemKind::Azalea => {
                vec![]
            }
            ItemKind::FloweringAzalea => {
                vec![]
            }
            ItemKind::DeadBush => {
                vec![]
            }
            ItemKind::Seagrass => {
                vec![]
            }
            ItemKind::SeaPickle => {
                vec![]
            }
            ItemKind::WhiteWool => {
                vec![]
            }
            ItemKind::OrangeWool => {
                vec![]
            }
            ItemKind::MagentaWool => {
                vec![]
            }
            ItemKind::LightBlueWool => {
                vec![]
            }
            ItemKind::YellowWool => {
                vec![]
            }
            ItemKind::LimeWool => {
                vec![]
            }
            ItemKind::PinkWool => {
                vec![]
            }
            ItemKind::GrayWool => {
                vec![]
            }
            ItemKind::LightGrayWool => {
                vec![]
            }
            ItemKind::CyanWool => {
                vec![]
            }
            ItemKind::PurpleWool => {
                vec![]
            }
            ItemKind::BlueWool => {
                vec![]
            }
            ItemKind::BrownWool => {
                vec![]
            }
            ItemKind::GreenWool => {
                vec![]
            }
            ItemKind::RedWool => {
                vec![]
            }
            ItemKind::BlackWool => {
                vec![]
            }
            ItemKind::Dandelion => {
                vec![]
            }
            ItemKind::Poppy => {
                vec![]
            }
            ItemKind::BlueOrchid => {
                vec![]
            }
            ItemKind::Allium => {
                vec![]
            }
            ItemKind::AzureBluet => {
                vec![]
            }
            ItemKind::RedTulip => {
                vec![]
            }
            ItemKind::OrangeTulip => {
                vec![]
            }
            ItemKind::WhiteTulip => {
                vec![]
            }
            ItemKind::PinkTulip => {
                vec![]
            }
            ItemKind::OxeyeDaisy => {
                vec![]
            }
            ItemKind::Cornflower => {
                vec![]
            }
            ItemKind::LilyOfTheValley => {
                vec![]
            }
            ItemKind::WitherRose => {
                vec![]
            }
            ItemKind::SporeBlossom => {
                vec![]
            }
            ItemKind::BrownMushroom => {
                vec![]
            }
            ItemKind::RedMushroom => {
                vec![]
            }
            ItemKind::CrimsonFungus => {
                vec![]
            }
            ItemKind::WarpedFungus => {
                vec![]
            }
            ItemKind::CrimsonRoots => {
                vec![]
            }
            ItemKind::WarpedRoots => {
                vec![]
            }
            ItemKind::NetherSprouts => {
                vec![]
            }
            ItemKind::WeepingVines => {
                vec![]
            }
            ItemKind::TwistingVines => {
                vec![]
            }
            ItemKind::SugarCane => {
                vec![]
            }
            ItemKind::Kelp => {
                vec![]
            }
            ItemKind::MossCarpet => {
                vec![]
            }
            ItemKind::MossBlock => {
                vec![]
            }
            ItemKind::HangingRoots => {
                vec![]
            }
            ItemKind::BigDripleaf => {
                vec![]
            }
            ItemKind::SmallDripleaf => {
                vec![]
            }
            ItemKind::Bamboo => {
                vec![]
            }
            ItemKind::OakSlab => {
                vec![]
            }
            ItemKind::SpruceSlab => {
                vec![]
            }
            ItemKind::BirchSlab => {
                vec![]
            }
            ItemKind::JungleSlab => {
                vec![]
            }
            ItemKind::AcaciaSlab => {
                vec![]
            }
            ItemKind::DarkOakSlab => {
                vec![]
            }
            ItemKind::CrimsonSlab => {
                vec![]
            }
            ItemKind::WarpedSlab => {
                vec![]
            }
            ItemKind::StoneSlab => {
                vec![]
            }
            ItemKind::SmoothStoneSlab => {
                vec![]
            }
            ItemKind::SandstoneSlab => {
                vec![]
            }
            ItemKind::CutSandstoneSlab => {
                vec![]
            }
            ItemKind::PetrifiedOakSlab => {
                vec![]
            }
            ItemKind::CobblestoneSlab => {
                vec![]
            }
            ItemKind::BrickSlab => {
                vec![]
            }
            ItemKind::StoneBrickSlab => {
                vec![]
            }
            ItemKind::NetherBrickSlab => {
                vec![]
            }
            ItemKind::QuartzSlab => {
                vec![]
            }
            ItemKind::RedSandstoneSlab => {
                vec![]
            }
            ItemKind::CutRedSandstoneSlab => {
                vec![]
            }
            ItemKind::PurpurSlab => {
                vec![]
            }
            ItemKind::PrismarineSlab => {
                vec![]
            }
            ItemKind::PrismarineBrickSlab => {
                vec![]
            }
            ItemKind::DarkPrismarineSlab => {
                vec![]
            }
            ItemKind::SmoothQuartz => {
                vec![]
            }
            ItemKind::SmoothRedSandstone => {
                vec![]
            }
            ItemKind::SmoothSandstone => {
                vec![]
            }
            ItemKind::SmoothStone => {
                vec![]
            }
            ItemKind::Bricks => {
                vec![]
            }
            ItemKind::Bookshelf => {
                vec![]
            }
            ItemKind::MossyCobblestone => {
                vec![]
            }
            ItemKind::Obsidian => {
                vec![]
            }
            ItemKind::Torch => {
                vec![]
            }
            ItemKind::EndRod => {
                vec![]
            }
            ItemKind::ChorusPlant => {
                vec![]
            }
            ItemKind::ChorusFlower => {
                vec![]
            }
            ItemKind::PurpurBlock => {
                vec![]
            }
            ItemKind::PurpurPillar => {
                vec![]
            }
            ItemKind::PurpurStairs => {
                vec![]
            }
            ItemKind::Spawner => {
                vec![]
            }
            ItemKind::OakStairs => {
                vec![]
            }
            ItemKind::Chest => {
                vec![]
            }
            ItemKind::CraftingTable => {
                vec![]
            }
            ItemKind::Farmland => {
                vec![]
            }
            ItemKind::Furnace => {
                vec![]
            }
            ItemKind::Ladder => {
                vec![]
            }
            ItemKind::CobblestoneStairs => {
                vec![]
            }
            ItemKind::Snow => {
                vec![]
            }
            ItemKind::Ice => {
                vec![]
            }
            ItemKind::SnowBlock => {
                vec![]
            }
            ItemKind::Cactus => {
                vec![]
            }
            ItemKind::Clay => {
                vec![]
            }
            ItemKind::Jukebox => {
                vec![]
            }
            ItemKind::OakFence => {
                vec![]
            }
            ItemKind::SpruceFence => {
                vec![]
            }
            ItemKind::BirchFence => {
                vec![]
            }
            ItemKind::JungleFence => {
                vec![]
            }
            ItemKind::AcaciaFence => {
                vec![]
            }
            ItemKind::DarkOakFence => {
                vec![]
            }
            ItemKind::CrimsonFence => {
                vec![]
            }
            ItemKind::WarpedFence => {
                vec![]
            }
            ItemKind::Pumpkin => {
                vec![]
            }
            ItemKind::CarvedPumpkin => {
                vec![]
            }
            ItemKind::JackOLantern => {
                vec![]
            }
            ItemKind::Netherrack => {
                vec![]
            }
            ItemKind::SoulSand => {
                vec![]
            }
            ItemKind::SoulSoil => {
                vec![]
            }
            ItemKind::Basalt => {
                vec![]
            }
            ItemKind::PolishedBasalt => {
                vec![]
            }
            ItemKind::SmoothBasalt => {
                vec![]
            }
            ItemKind::SoulTorch => {
                vec![]
            }
            ItemKind::Glowstone => {
                vec![]
            }
            ItemKind::InfestedStone => {
                vec![]
            }
            ItemKind::InfestedCobblestone => {
                vec![]
            }
            ItemKind::InfestedStoneBricks => {
                vec![]
            }
            ItemKind::InfestedMossyStoneBricks => {
                vec![]
            }
            ItemKind::InfestedCrackedStoneBricks => {
                vec![]
            }
            ItemKind::InfestedChiseledStoneBricks => {
                vec![]
            }
            ItemKind::InfestedDeepslate => {
                vec![]
            }
            ItemKind::StoneBricks => {
                vec![]
            }
            ItemKind::MossyStoneBricks => {
                vec![]
            }
            ItemKind::CrackedStoneBricks => {
                vec![]
            }
            ItemKind::ChiseledStoneBricks => {
                vec![]
            }
            ItemKind::DeepslateBricks => {
                vec![]
            }
            ItemKind::CrackedDeepslateBricks => {
                vec![]
            }
            ItemKind::DeepslateTiles => {
                vec![]
            }
            ItemKind::CrackedDeepslateTiles => {
                vec![]
            }
            ItemKind::ChiseledDeepslate => {
                vec![]
            }
            ItemKind::BrownMushroomBlock => {
                vec![]
            }
            ItemKind::RedMushroomBlock => {
                vec![]
            }
            ItemKind::MushroomStem => {
                vec![]
            }
            ItemKind::IronBars => {
                vec![]
            }
            ItemKind::Chain => {
                vec![]
            }
            ItemKind::GlassPane => {
                vec![]
            }
            ItemKind::Melon => {
                vec![]
            }
            ItemKind::Vine => {
                vec![]
            }
            ItemKind::GlowLichen => {
                vec![]
            }
            ItemKind::BrickStairs => {
                vec![]
            }
            ItemKind::StoneBrickStairs => {
                vec![]
            }
            ItemKind::Mycelium => {
                vec![]
            }
            ItemKind::LilyPad => {
                vec![]
            }
            ItemKind::NetherBricks => {
                vec![]
            }
            ItemKind::CrackedNetherBricks => {
                vec![]
            }
            ItemKind::ChiseledNetherBricks => {
                vec![]
            }
            ItemKind::NetherBrickFence => {
                vec![]
            }
            ItemKind::NetherBrickStairs => {
                vec![]
            }
            ItemKind::EnchantingTable => {
                vec![]
            }
            ItemKind::EndPortalFrame => {
                vec![]
            }
            ItemKind::EndStone => {
                vec![]
            }
            ItemKind::EndStoneBricks => {
                vec![]
            }
            ItemKind::DragonEgg => {
                vec![]
            }
            ItemKind::SandstoneStairs => {
                vec![]
            }
            ItemKind::EnderChest => {
                vec![]
            }
            ItemKind::EmeraldBlock => {
                vec![]
            }
            ItemKind::SpruceStairs => {
                vec![]
            }
            ItemKind::BirchStairs => {
                vec![]
            }
            ItemKind::JungleStairs => {
                vec![]
            }
            ItemKind::CrimsonStairs => {
                vec![]
            }
            ItemKind::WarpedStairs => {
                vec![]
            }
            ItemKind::CommandBlock => {
                vec![]
            }
            ItemKind::Beacon => {
                vec![]
            }
            ItemKind::CobblestoneWall => {
                vec![]
            }
            ItemKind::MossyCobblestoneWall => {
                vec![]
            }
            ItemKind::BrickWall => {
                vec![]
            }
            ItemKind::PrismarineWall => {
                vec![]
            }
            ItemKind::RedSandstoneWall => {
                vec![]
            }
            ItemKind::MossyStoneBrickWall => {
                vec![]
            }
            ItemKind::GraniteWall => {
                vec![]
            }
            ItemKind::StoneBrickWall => {
                vec![]
            }
            ItemKind::NetherBrickWall => {
                vec![]
            }
            ItemKind::AndesiteWall => {
                vec![]
            }
            ItemKind::RedNetherBrickWall => {
                vec![]
            }
            ItemKind::SandstoneWall => {
                vec![]
            }
            ItemKind::EndStoneBrickWall => {
                vec![]
            }
            ItemKind::DioriteWall => {
                vec![]
            }
            ItemKind::BlackstoneWall => {
                vec![]
            }
            ItemKind::PolishedBlackstoneWall => {
                vec![]
            }
            ItemKind::PolishedBlackstoneBrickWall => {
                vec![]
            }
            ItemKind::CobbledDeepslateWall => {
                vec![]
            }
            ItemKind::PolishedDeepslateWall => {
                vec![]
            }
            ItemKind::DeepslateBrickWall => {
                vec![]
            }
            ItemKind::DeepslateTileWall => {
                vec![]
            }
            ItemKind::Anvil => {
                vec![]
            }
            ItemKind::ChippedAnvil => {
                vec![]
            }
            ItemKind::DamagedAnvil => {
                vec![]
            }
            ItemKind::ChiseledQuartzBlock => {
                vec![]
            }
            ItemKind::QuartzBlock => {
                vec![]
            }
            ItemKind::QuartzBricks => {
                vec![]
            }
            ItemKind::QuartzPillar => {
                vec![]
            }
            ItemKind::QuartzStairs => {
                vec![]
            }
            ItemKind::WhiteTerracotta => {
                vec![]
            }
            ItemKind::OrangeTerracotta => {
                vec![]
            }
            ItemKind::MagentaTerracotta => {
                vec![]
            }
            ItemKind::LightBlueTerracotta => {
                vec![]
            }
            ItemKind::YellowTerracotta => {
                vec![]
            }
            ItemKind::LimeTerracotta => {
                vec![]
            }
            ItemKind::PinkTerracotta => {
                vec![]
            }
            ItemKind::GrayTerracotta => {
                vec![]
            }
            ItemKind::LightGrayTerracotta => {
                vec![]
            }
            ItemKind::CyanTerracotta => {
                vec![]
            }
            ItemKind::PurpleTerracotta => {
                vec![]
            }
            ItemKind::BlueTerracotta => {
                vec![]
            }
            ItemKind::BrownTerracotta => {
                vec![]
            }
            ItemKind::GreenTerracotta => {
                vec![]
            }
            ItemKind::RedTerracotta => {
                vec![]
            }
            ItemKind::BlackTerracotta => {
                vec![]
            }
            ItemKind::Barrier => {
                vec![]
            }
            ItemKind::Light => {
                vec![]
            }
            ItemKind::HayBlock => {
                vec![]
            }
            ItemKind::WhiteCarpet => {
                vec![]
            }
            ItemKind::OrangeCarpet => {
                vec![]
            }
            ItemKind::MagentaCarpet => {
                vec![]
            }
            ItemKind::LightBlueCarpet => {
                vec![]
            }
            ItemKind::YellowCarpet => {
                vec![]
            }
            ItemKind::LimeCarpet => {
                vec![]
            }
            ItemKind::PinkCarpet => {
                vec![]
            }
            ItemKind::GrayCarpet => {
                vec![]
            }
            ItemKind::LightGrayCarpet => {
                vec![]
            }
            ItemKind::CyanCarpet => {
                vec![]
            }
            ItemKind::PurpleCarpet => {
                vec![]
            }
            ItemKind::BlueCarpet => {
                vec![]
            }
            ItemKind::BrownCarpet => {
                vec![]
            }
            ItemKind::GreenCarpet => {
                vec![]
            }
            ItemKind::RedCarpet => {
                vec![]
            }
            ItemKind::BlackCarpet => {
                vec![]
            }
            ItemKind::Terracotta => {
                vec![]
            }
            ItemKind::PackedIce => {
                vec![]
            }
            ItemKind::AcaciaStairs => {
                vec![]
            }
            ItemKind::DarkOakStairs => {
                vec![]
            }
            ItemKind::DirtPath => {
                vec![]
            }
            ItemKind::Sunflower => {
                vec![]
            }
            ItemKind::Lilac => {
                vec![]
            }
            ItemKind::RoseBush => {
                vec![]
            }
            ItemKind::Peony => {
                vec![]
            }
            ItemKind::TallGrass => {
                vec![]
            }
            ItemKind::LargeFern => {
                vec![]
            }
            ItemKind::WhiteStainedGlass => {
                vec![]
            }
            ItemKind::OrangeStainedGlass => {
                vec![]
            }
            ItemKind::MagentaStainedGlass => {
                vec![]
            }
            ItemKind::LightBlueStainedGlass => {
                vec![]
            }
            ItemKind::YellowStainedGlass => {
                vec![]
            }
            ItemKind::LimeStainedGlass => {
                vec![]
            }
            ItemKind::PinkStainedGlass => {
                vec![]
            }
            ItemKind::GrayStainedGlass => {
                vec![]
            }
            ItemKind::LightGrayStainedGlass => {
                vec![]
            }
            ItemKind::CyanStainedGlass => {
                vec![]
            }
            ItemKind::PurpleStainedGlass => {
                vec![]
            }
            ItemKind::BlueStainedGlass => {
                vec![]
            }
            ItemKind::BrownStainedGlass => {
                vec![]
            }
            ItemKind::GreenStainedGlass => {
                vec![]
            }
            ItemKind::RedStainedGlass => {
                vec![]
            }
            ItemKind::BlackStainedGlass => {
                vec![]
            }
            ItemKind::WhiteStainedGlassPane => {
                vec![]
            }
            ItemKind::OrangeStainedGlassPane => {
                vec![]
            }
            ItemKind::MagentaStainedGlassPane => {
                vec![]
            }
            ItemKind::LightBlueStainedGlassPane => {
                vec![]
            }
            ItemKind::YellowStainedGlassPane => {
                vec![]
            }
            ItemKind::LimeStainedGlassPane => {
                vec![]
            }
            ItemKind::PinkStainedGlassPane => {
                vec![]
            }
            ItemKind::GrayStainedGlassPane => {
                vec![]
            }
            ItemKind::LightGrayStainedGlassPane => {
                vec![]
            }
            ItemKind::CyanStainedGlassPane => {
                vec![]
            }
            ItemKind::PurpleStainedGlassPane => {
                vec![]
            }
            ItemKind::BlueStainedGlassPane => {
                vec![]
            }
            ItemKind::BrownStainedGlassPane => {
                vec![]
            }
            ItemKind::GreenStainedGlassPane => {
                vec![]
            }
            ItemKind::RedStainedGlassPane => {
                vec![]
            }
            ItemKind::BlackStainedGlassPane => {
                vec![]
            }
            ItemKind::Prismarine => {
                vec![]
            }
            ItemKind::PrismarineBricks => {
                vec![]
            }
            ItemKind::DarkPrismarine => {
                vec![]
            }
            ItemKind::PrismarineStairs => {
                vec![]
            }
            ItemKind::PrismarineBrickStairs => {
                vec![]
            }
            ItemKind::DarkPrismarineStairs => {
                vec![]
            }
            ItemKind::SeaLantern => {
                vec![]
            }
            ItemKind::RedSandstone => {
                vec![]
            }
            ItemKind::ChiseledRedSandstone => {
                vec![]
            }
            ItemKind::CutRedSandstone => {
                vec![]
            }
            ItemKind::RedSandstoneStairs => {
                vec![]
            }
            ItemKind::RepeatingCommandBlock => {
                vec![]
            }
            ItemKind::ChainCommandBlock => {
                vec![]
            }
            ItemKind::MagmaBlock => {
                vec![]
            }
            ItemKind::NetherWartBlock => {
                vec![]
            }
            ItemKind::WarpedWartBlock => {
                vec![]
            }
            ItemKind::RedNetherBricks => {
                vec![]
            }
            ItemKind::BoneBlock => {
                vec![]
            }
            ItemKind::StructureVoid => {
                vec![]
            }
            ItemKind::ShulkerBox => {
                vec![]
            }
            ItemKind::WhiteShulkerBox => {
                vec![]
            }
            ItemKind::OrangeShulkerBox => {
                vec![]
            }
            ItemKind::MagentaShulkerBox => {
                vec![]
            }
            ItemKind::LightBlueShulkerBox => {
                vec![]
            }
            ItemKind::YellowShulkerBox => {
                vec![]
            }
            ItemKind::LimeShulkerBox => {
                vec![]
            }
            ItemKind::PinkShulkerBox => {
                vec![]
            }
            ItemKind::GrayShulkerBox => {
                vec![]
            }
            ItemKind::LightGrayShulkerBox => {
                vec![]
            }
            ItemKind::CyanShulkerBox => {
                vec![]
            }
            ItemKind::PurpleShulkerBox => {
                vec![]
            }
            ItemKind::BlueShulkerBox => {
                vec![]
            }
            ItemKind::BrownShulkerBox => {
                vec![]
            }
            ItemKind::GreenShulkerBox => {
                vec![]
            }
            ItemKind::RedShulkerBox => {
                vec![]
            }
            ItemKind::BlackShulkerBox => {
                vec![]
            }
            ItemKind::WhiteGlazedTerracotta => {
                vec![]
            }
            ItemKind::OrangeGlazedTerracotta => {
                vec![]
            }
            ItemKind::MagentaGlazedTerracotta => {
                vec![]
            }
            ItemKind::LightBlueGlazedTerracotta => {
                vec![]
            }
            ItemKind::YellowGlazedTerracotta => {
                vec![]
            }
            ItemKind::LimeGlazedTerracotta => {
                vec![]
            }
            ItemKind::PinkGlazedTerracotta => {
                vec![]
            }
            ItemKind::GrayGlazedTerracotta => {
                vec![]
            }
            ItemKind::LightGrayGlazedTerracotta => {
                vec![]
            }
            ItemKind::CyanGlazedTerracotta => {
                vec![]
            }
            ItemKind::PurpleGlazedTerracotta => {
                vec![]
            }
            ItemKind::BlueGlazedTerracotta => {
                vec![]
            }
            ItemKind::BrownGlazedTerracotta => {
                vec![]
            }
            ItemKind::GreenGlazedTerracotta => {
                vec![]
            }
            ItemKind::RedGlazedTerracotta => {
                vec![]
            }
            ItemKind::BlackGlazedTerracotta => {
                vec![]
            }
            ItemKind::WhiteConcrete => {
                vec![]
            }
            ItemKind::OrangeConcrete => {
                vec![]
            }
            ItemKind::MagentaConcrete => {
                vec![]
            }
            ItemKind::LightBlueConcrete => {
                vec![]
            }
            ItemKind::YellowConcrete => {
                vec![]
            }
            ItemKind::LimeConcrete => {
                vec![]
            }
            ItemKind::PinkConcrete => {
                vec![]
            }
            ItemKind::GrayConcrete => {
                vec![]
            }
            ItemKind::LightGrayConcrete => {
                vec![]
            }
            ItemKind::CyanConcrete => {
                vec![]
            }
            ItemKind::PurpleConcrete => {
                vec![]
            }
            ItemKind::BlueConcrete => {
                vec![]
            }
            ItemKind::BrownConcrete => {
                vec![]
            }
            ItemKind::GreenConcrete => {
                vec![]
            }
            ItemKind::RedConcrete => {
                vec![]
            }
            ItemKind::BlackConcrete => {
                vec![]
            }
            ItemKind::WhiteConcretePowder => {
                vec![]
            }
            ItemKind::OrangeConcretePowder => {
                vec![]
            }
            ItemKind::MagentaConcretePowder => {
                vec![]
            }
            ItemKind::LightBlueConcretePowder => {
                vec![]
            }
            ItemKind::YellowConcretePowder => {
                vec![]
            }
            ItemKind::LimeConcretePowder => {
                vec![]
            }
            ItemKind::PinkConcretePowder => {
                vec![]
            }
            ItemKind::GrayConcretePowder => {
                vec![]
            }
            ItemKind::LightGrayConcretePowder => {
                vec![]
            }
            ItemKind::CyanConcretePowder => {
                vec![]
            }
            ItemKind::PurpleConcretePowder => {
                vec![]
            }
            ItemKind::BlueConcretePowder => {
                vec![]
            }
            ItemKind::BrownConcretePowder => {
                vec![]
            }
            ItemKind::GreenConcretePowder => {
                vec![]
            }
            ItemKind::RedConcretePowder => {
                vec![]
            }
            ItemKind::BlackConcretePowder => {
                vec![]
            }
            ItemKind::TurtleEgg => {
                vec![]
            }
            ItemKind::DeadTubeCoralBlock => {
                vec![]
            }
            ItemKind::DeadBrainCoralBlock => {
                vec![]
            }
            ItemKind::DeadBubbleCoralBlock => {
                vec![]
            }
            ItemKind::DeadFireCoralBlock => {
                vec![]
            }
            ItemKind::DeadHornCoralBlock => {
                vec![]
            }
            ItemKind::TubeCoralBlock => {
                vec![]
            }
            ItemKind::BrainCoralBlock => {
                vec![]
            }
            ItemKind::BubbleCoralBlock => {
                vec![]
            }
            ItemKind::FireCoralBlock => {
                vec![]
            }
            ItemKind::HornCoralBlock => {
                vec![]
            }
            ItemKind::TubeCoral => {
                vec![]
            }
            ItemKind::BrainCoral => {
                vec![]
            }
            ItemKind::BubbleCoral => {
                vec![]
            }
            ItemKind::FireCoral => {
                vec![]
            }
            ItemKind::HornCoral => {
                vec![]
            }
            ItemKind::DeadBrainCoral => {
                vec![]
            }
            ItemKind::DeadBubbleCoral => {
                vec![]
            }
            ItemKind::DeadFireCoral => {
                vec![]
            }
            ItemKind::DeadHornCoral => {
                vec![]
            }
            ItemKind::DeadTubeCoral => {
                vec![]
            }
            ItemKind::TubeCoralFan => {
                vec![]
            }
            ItemKind::BrainCoralFan => {
                vec![]
            }
            ItemKind::BubbleCoralFan => {
                vec![]
            }
            ItemKind::FireCoralFan => {
                vec![]
            }
            ItemKind::HornCoralFan => {
                vec![]
            }
            ItemKind::DeadTubeCoralFan => {
                vec![]
            }
            ItemKind::DeadBrainCoralFan => {
                vec![]
            }
            ItemKind::DeadBubbleCoralFan => {
                vec![]
            }
            ItemKind::DeadFireCoralFan => {
                vec![]
            }
            ItemKind::DeadHornCoralFan => {
                vec![]
            }
            ItemKind::BlueIce => {
                vec![]
            }
            ItemKind::Conduit => {
                vec![]
            }
            ItemKind::PolishedGraniteStairs => {
                vec![]
            }
            ItemKind::SmoothRedSandstoneStairs => {
                vec![]
            }
            ItemKind::MossyStoneBrickStairs => {
                vec![]
            }
            ItemKind::PolishedDioriteStairs => {
                vec![]
            }
            ItemKind::MossyCobblestoneStairs => {
                vec![]
            }
            ItemKind::EndStoneBrickStairs => {
                vec![]
            }
            ItemKind::StoneStairs => {
                vec![]
            }
            ItemKind::SmoothSandstoneStairs => {
                vec![]
            }
            ItemKind::SmoothQuartzStairs => {
                vec![]
            }
            ItemKind::GraniteStairs => {
                vec![]
            }
            ItemKind::AndesiteStairs => {
                vec![]
            }
            ItemKind::RedNetherBrickStairs => {
                vec![]
            }
            ItemKind::PolishedAndesiteStairs => {
                vec![]
            }
            ItemKind::DioriteStairs => {
                vec![]
            }
            ItemKind::CobbledDeepslateStairs => {
                vec![]
            }
            ItemKind::PolishedDeepslateStairs => {
                vec![]
            }
            ItemKind::DeepslateBrickStairs => {
                vec![]
            }
            ItemKind::DeepslateTileStairs => {
                vec![]
            }
            ItemKind::PolishedGraniteSlab => {
                vec![]
            }
            ItemKind::SmoothRedSandstoneSlab => {
                vec![]
            }
            ItemKind::MossyStoneBrickSlab => {
                vec![]
            }
            ItemKind::PolishedDioriteSlab => {
                vec![]
            }
            ItemKind::MossyCobblestoneSlab => {
                vec![]
            }
            ItemKind::EndStoneBrickSlab => {
                vec![]
            }
            ItemKind::SmoothSandstoneSlab => {
                vec![]
            }
            ItemKind::SmoothQuartzSlab => {
                vec![]
            }
            ItemKind::GraniteSlab => {
                vec![]
            }
            ItemKind::AndesiteSlab => {
                vec![]
            }
            ItemKind::RedNetherBrickSlab => {
                vec![]
            }
            ItemKind::PolishedAndesiteSlab => {
                vec![]
            }
            ItemKind::DioriteSlab => {
                vec![]
            }
            ItemKind::CobbledDeepslateSlab => {
                vec![]
            }
            ItemKind::PolishedDeepslateSlab => {
                vec![]
            }
            ItemKind::DeepslateBrickSlab => {
                vec![]
            }
            ItemKind::DeepslateTileSlab => {
                vec![]
            }
            ItemKind::Scaffolding => {
                vec![]
            }
            ItemKind::Redstone => {
                vec![]
            }
            ItemKind::RedstoneTorch => {
                vec![]
            }
            ItemKind::RedstoneBlock => {
                vec![]
            }
            ItemKind::Repeater => {
                vec![]
            }
            ItemKind::Comparator => {
                vec![]
            }
            ItemKind::Piston => {
                vec![]
            }
            ItemKind::StickyPiston => {
                vec![]
            }
            ItemKind::SlimeBlock => {
                vec![]
            }
            ItemKind::HoneyBlock => {
                vec![]
            }
            ItemKind::Observer => {
                vec![]
            }
            ItemKind::Hopper => {
                vec![]
            }
            ItemKind::Dispenser => {
                vec![]
            }
            ItemKind::Dropper => {
                vec![]
            }
            ItemKind::Lectern => {
                vec![]
            }
            ItemKind::Target => {
                vec![]
            }
            ItemKind::Lever => {
                vec![]
            }
            ItemKind::LightningRod => {
                vec![]
            }
            ItemKind::DaylightDetector => {
                vec![]
            }
            ItemKind::SculkSensor => {
                vec![]
            }
            ItemKind::TripwireHook => {
                vec![]
            }
            ItemKind::TrappedChest => {
                vec![]
            }
            ItemKind::Tnt => {
                vec![]
            }
            ItemKind::RedstoneLamp => {
                vec![]
            }
            ItemKind::NoteBlock => {
                vec![]
            }
            ItemKind::StoneButton => {
                vec![]
            }
            ItemKind::PolishedBlackstoneButton => {
                vec![]
            }
            ItemKind::OakButton => {
                vec![]
            }
            ItemKind::SpruceButton => {
                vec![]
            }
            ItemKind::BirchButton => {
                vec![]
            }
            ItemKind::JungleButton => {
                vec![]
            }
            ItemKind::AcaciaButton => {
                vec![]
            }
            ItemKind::DarkOakButton => {
                vec![]
            }
            ItemKind::CrimsonButton => {
                vec![]
            }
            ItemKind::WarpedButton => {
                vec![]
            }
            ItemKind::StonePressurePlate => {
                vec![]
            }
            ItemKind::PolishedBlackstonePressurePlate => {
                vec![]
            }
            ItemKind::LightWeightedPressurePlate => {
                vec![]
            }
            ItemKind::HeavyWeightedPressurePlate => {
                vec![]
            }
            ItemKind::OakPressurePlate => {
                vec![]
            }
            ItemKind::SprucePressurePlate => {
                vec![]
            }
            ItemKind::BirchPressurePlate => {
                vec![]
            }
            ItemKind::JunglePressurePlate => {
                vec![]
            }
            ItemKind::AcaciaPressurePlate => {
                vec![]
            }
            ItemKind::DarkOakPressurePlate => {
                vec![]
            }
            ItemKind::CrimsonPressurePlate => {
                vec![]
            }
            ItemKind::WarpedPressurePlate => {
                vec![]
            }
            ItemKind::IronDoor => {
                vec![]
            }
            ItemKind::OakDoor => {
                vec![]
            }
            ItemKind::SpruceDoor => {
                vec![]
            }
            ItemKind::BirchDoor => {
                vec![]
            }
            ItemKind::JungleDoor => {
                vec![]
            }
            ItemKind::AcaciaDoor => {
                vec![]
            }
            ItemKind::DarkOakDoor => {
                vec![]
            }
            ItemKind::CrimsonDoor => {
                vec![]
            }
            ItemKind::WarpedDoor => {
                vec![]
            }
            ItemKind::IronTrapdoor => {
                vec![]
            }
            ItemKind::OakTrapdoor => {
                vec![]
            }
            ItemKind::SpruceTrapdoor => {
                vec![]
            }
            ItemKind::BirchTrapdoor => {
                vec![]
            }
            ItemKind::JungleTrapdoor => {
                vec![]
            }
            ItemKind::AcaciaTrapdoor => {
                vec![]
            }
            ItemKind::DarkOakTrapdoor => {
                vec![]
            }
            ItemKind::CrimsonTrapdoor => {
                vec![]
            }
            ItemKind::WarpedTrapdoor => {
                vec![]
            }
            ItemKind::OakFenceGate => {
                vec![]
            }
            ItemKind::SpruceFenceGate => {
                vec![]
            }
            ItemKind::BirchFenceGate => {
                vec![]
            }
            ItemKind::JungleFenceGate => {
                vec![]
            }
            ItemKind::AcaciaFenceGate => {
                vec![]
            }
            ItemKind::DarkOakFenceGate => {
                vec![]
            }
            ItemKind::CrimsonFenceGate => {
                vec![]
            }
            ItemKind::WarpedFenceGate => {
                vec![]
            }
            ItemKind::PoweredRail => {
                vec![]
            }
            ItemKind::DetectorRail => {
                vec![]
            }
            ItemKind::Rail => {
                vec![]
            }
            ItemKind::ActivatorRail => {
                vec![]
            }
            ItemKind::Saddle => {
                vec![]
            }
            ItemKind::Minecart => {
                vec![]
            }
            ItemKind::ChestMinecart => {
                vec![]
            }
            ItemKind::FurnaceMinecart => {
                vec![]
            }
            ItemKind::TntMinecart => {
                vec![]
            }
            ItemKind::HopperMinecart => {
                vec![]
            }
            ItemKind::CarrotOnAStick => {
                vec![]
            }
            ItemKind::WarpedFungusOnAStick => {
                vec![]
            }
            ItemKind::Elytra => {
                vec![]
            }
            ItemKind::OakBoat => {
                vec![]
            }
            ItemKind::SpruceBoat => {
                vec![]
            }
            ItemKind::BirchBoat => {
                vec![]
            }
            ItemKind::JungleBoat => {
                vec![]
            }
            ItemKind::AcaciaBoat => {
                vec![]
            }
            ItemKind::DarkOakBoat => {
                vec![]
            }
            ItemKind::StructureBlock => {
                vec![]
            }
            ItemKind::Jigsaw => {
                vec![]
            }
            ItemKind::TurtleHelmet => {
                vec![]
            }
            ItemKind::Scute => {
                vec![]
            }
            ItemKind::FlintAndSteel => {
                vec![]
            }
            ItemKind::Apple => {
                vec![]
            }
            ItemKind::Bow => {
                vec![]
            }
            ItemKind::Arrow => {
                vec![]
            }
            ItemKind::Coal => {
                vec![]
            }
            ItemKind::Charcoal => {
                vec![]
            }
            ItemKind::Diamond => {
                vec![]
            }
            ItemKind::Emerald => {
                vec![]
            }
            ItemKind::LapisLazuli => {
                vec![]
            }
            ItemKind::Quartz => {
                vec![]
            }
            ItemKind::AmethystShard => {
                vec![]
            }
            ItemKind::RawIron => {
                vec![]
            }
            ItemKind::IronIngot => {
                vec![]
            }
            ItemKind::RawCopper => {
                vec![]
            }
            ItemKind::CopperIngot => {
                vec![]
            }
            ItemKind::RawGold => {
                vec![]
            }
            ItemKind::GoldIngot => {
                vec![]
            }
            ItemKind::NetheriteIngot => {
                vec![]
            }
            ItemKind::NetheriteScrap => {
                vec![]
            }
            ItemKind::WoodenSword => {
                vec![]
            }
            ItemKind::WoodenShovel => {
                vec![]
            }
            ItemKind::WoodenPickaxe => {
                vec![]
            }
            ItemKind::WoodenAxe => {
                vec![]
            }
            ItemKind::WoodenHoe => {
                vec![]
            }
            ItemKind::StoneSword => {
                vec![]
            }
            ItemKind::StoneShovel => {
                vec![]
            }
            ItemKind::StonePickaxe => {
                vec![]
            }
            ItemKind::StoneAxe => {
                vec![]
            }
            ItemKind::StoneHoe => {
                vec![]
            }
            ItemKind::GoldenSword => {
                vec![]
            }
            ItemKind::GoldenShovel => {
                vec![]
            }
            ItemKind::GoldenPickaxe => {
                vec![]
            }
            ItemKind::GoldenAxe => {
                vec![]
            }
            ItemKind::GoldenHoe => {
                vec![]
            }
            ItemKind::IronSword => {
                vec![]
            }
            ItemKind::IronShovel => {
                vec![]
            }
            ItemKind::IronPickaxe => {
                vec![]
            }
            ItemKind::IronAxe => {
                vec![]
            }
            ItemKind::IronHoe => {
                vec![]
            }
            ItemKind::DiamondSword => {
                vec![]
            }
            ItemKind::DiamondShovel => {
                vec![]
            }
            ItemKind::DiamondPickaxe => {
                vec![]
            }
            ItemKind::DiamondAxe => {
                vec![]
            }
            ItemKind::DiamondHoe => {
                vec![]
            }
            ItemKind::NetheriteSword => {
                vec![]
            }
            ItemKind::NetheriteShovel => {
                vec![]
            }
            ItemKind::NetheritePickaxe => {
                vec![]
            }
            ItemKind::NetheriteAxe => {
                vec![]
            }
            ItemKind::NetheriteHoe => {
                vec![]
            }
            ItemKind::Stick => {
                vec![]
            }
            ItemKind::Bowl => {
                vec![]
            }
            ItemKind::MushroomStew => {
                vec![]
            }
            ItemKind::String => {
                vec![]
            }
            ItemKind::Feather => {
                vec![]
            }
            ItemKind::Gunpowder => {
                vec![]
            }
            ItemKind::WheatSeeds => {
                vec![]
            }
            ItemKind::Wheat => {
                vec![]
            }
            ItemKind::Bread => {
                vec![]
            }
            ItemKind::LeatherHelmet => {
                vec![]
            }
            ItemKind::LeatherChestplate => {
                vec![]
            }
            ItemKind::LeatherLeggings => {
                vec![]
            }
            ItemKind::LeatherBoots => {
                vec![]
            }
            ItemKind::ChainmailHelmet => {
                vec![]
            }
            ItemKind::ChainmailChestplate => {
                vec![]
            }
            ItemKind::ChainmailLeggings => {
                vec![]
            }
            ItemKind::ChainmailBoots => {
                vec![]
            }
            ItemKind::IronHelmet => {
                vec![]
            }
            ItemKind::IronChestplate => {
                vec![]
            }
            ItemKind::IronLeggings => {
                vec![]
            }
            ItemKind::IronBoots => {
                vec![]
            }
            ItemKind::DiamondHelmet => {
                vec![]
            }
            ItemKind::DiamondChestplate => {
                vec![]
            }
            ItemKind::DiamondLeggings => {
                vec![]
            }
            ItemKind::DiamondBoots => {
                vec![]
            }
            ItemKind::GoldenHelmet => {
                vec![]
            }
            ItemKind::GoldenChestplate => {
                vec![]
            }
            ItemKind::GoldenLeggings => {
                vec![]
            }
            ItemKind::GoldenBoots => {
                vec![]
            }
            ItemKind::NetheriteHelmet => {
                vec![]
            }
            ItemKind::NetheriteChestplate => {
                vec![]
            }
            ItemKind::NetheriteLeggings => {
                vec![]
            }
            ItemKind::NetheriteBoots => {
                vec![]
            }
            ItemKind::Flint => {
                vec![]
            }
            ItemKind::Porkchop => {
                vec![]
            }
            ItemKind::CookedPorkchop => {
                vec![]
            }
            ItemKind::Painting => {
                vec![]
            }
            ItemKind::GoldenApple => {
                vec![]
            }
            ItemKind::EnchantedGoldenApple => {
                vec![]
            }
            ItemKind::OakSign => {
                vec![]
            }
            ItemKind::SpruceSign => {
                vec![]
            }
            ItemKind::BirchSign => {
                vec![]
            }
            ItemKind::JungleSign => {
                vec![]
            }
            ItemKind::AcaciaSign => {
                vec![]
            }
            ItemKind::DarkOakSign => {
                vec![]
            }
            ItemKind::CrimsonSign => {
                vec![]
            }
            ItemKind::WarpedSign => {
                vec![]
            }
            ItemKind::Bucket => {
                vec![]
            }
            ItemKind::WaterBucket => {
                vec![]
            }
            ItemKind::LavaBucket => {
                vec![]
            }
            ItemKind::PowderSnowBucket => {
                vec![]
            }
            ItemKind::Snowball => {
                vec![]
            }
            ItemKind::Leather => {
                vec![]
            }
            ItemKind::MilkBucket => {
                vec![]
            }
            ItemKind::PufferfishBucket => {
                vec![]
            }
            ItemKind::SalmonBucket => {
                vec![]
            }
            ItemKind::CodBucket => {
                vec![]
            }
            ItemKind::TropicalFishBucket => {
                vec![]
            }
            ItemKind::AxolotlBucket => {
                vec![]
            }
            ItemKind::Brick => {
                vec![]
            }
            ItemKind::ClayBall => {
                vec![]
            }
            ItemKind::DriedKelpBlock => {
                vec![]
            }
            ItemKind::Paper => {
                vec![]
            }
            ItemKind::Book => {
                vec![]
            }
            ItemKind::SlimeBall => {
                vec![]
            }
            ItemKind::Egg => {
                vec![]
            }
            ItemKind::Compass => {
                vec![]
            }
            ItemKind::Bundle => {
                vec![]
            }
            ItemKind::FishingRod => {
                vec![]
            }
            ItemKind::Clock => {
                vec![]
            }
            ItemKind::Spyglass => {
                vec![]
            }
            ItemKind::GlowstoneDust => {
                vec![]
            }
            ItemKind::Cod => {
                vec![]
            }
            ItemKind::Salmon => {
                vec![]
            }
            ItemKind::TropicalFish => {
                vec![]
            }
            ItemKind::Pufferfish => {
                vec![]
            }
            ItemKind::CookedCod => {
                vec![]
            }
            ItemKind::CookedSalmon => {
                vec![]
            }
            ItemKind::InkSac => {
                vec![]
            }
            ItemKind::GlowInkSac => {
                vec![]
            }
            ItemKind::CocoaBeans => {
                vec![]
            }
            ItemKind::WhiteDye => {
                vec![]
            }
            ItemKind::OrangeDye => {
                vec![]
            }
            ItemKind::MagentaDye => {
                vec![]
            }
            ItemKind::LightBlueDye => {
                vec![]
            }
            ItemKind::YellowDye => {
                vec![]
            }
            ItemKind::LimeDye => {
                vec![]
            }
            ItemKind::PinkDye => {
                vec![]
            }
            ItemKind::GrayDye => {
                vec![]
            }
            ItemKind::LightGrayDye => {
                vec![]
            }
            ItemKind::CyanDye => {
                vec![]
            }
            ItemKind::PurpleDye => {
                vec![]
            }
            ItemKind::BlueDye => {
                vec![]
            }
            ItemKind::BrownDye => {
                vec![]
            }
            ItemKind::GreenDye => {
                vec![]
            }
            ItemKind::RedDye => {
                vec![]
            }
            ItemKind::BlackDye => {
                vec![]
            }
            ItemKind::BoneMeal => {
                vec![]
            }
            ItemKind::Bone => {
                vec![]
            }
            ItemKind::Sugar => {
                vec![]
            }
            ItemKind::Cake => {
                vec![]
            }
            ItemKind::WhiteBed => {
                vec![]
            }
            ItemKind::OrangeBed => {
                vec![]
            }
            ItemKind::MagentaBed => {
                vec![]
            }
            ItemKind::LightBlueBed => {
                vec![]
            }
            ItemKind::YellowBed => {
                vec![]
            }
            ItemKind::LimeBed => {
                vec![]
            }
            ItemKind::PinkBed => {
                vec![]
            }
            ItemKind::GrayBed => {
                vec![]
            }
            ItemKind::LightGrayBed => {
                vec![]
            }
            ItemKind::CyanBed => {
                vec![]
            }
            ItemKind::PurpleBed => {
                vec![]
            }
            ItemKind::BlueBed => {
                vec![]
            }
            ItemKind::BrownBed => {
                vec![]
            }
            ItemKind::GreenBed => {
                vec![]
            }
            ItemKind::RedBed => {
                vec![]
            }
            ItemKind::BlackBed => {
                vec![]
            }
            ItemKind::Cookie => {
                vec![]
            }
            ItemKind::FilledMap => {
                vec![]
            }
            ItemKind::Shears => {
                vec![]
            }
            ItemKind::MelonSlice => {
                vec![]
            }
            ItemKind::DriedKelp => {
                vec![]
            }
            ItemKind::PumpkinSeeds => {
                vec![]
            }
            ItemKind::MelonSeeds => {
                vec![]
            }
            ItemKind::Beef => {
                vec![]
            }
            ItemKind::CookedBeef => {
                vec![]
            }
            ItemKind::Chicken => {
                vec![]
            }
            ItemKind::CookedChicken => {
                vec![]
            }
            ItemKind::RottenFlesh => {
                vec![]
            }
            ItemKind::EnderPearl => {
                vec![]
            }
            ItemKind::BlazeRod => {
                vec![]
            }
            ItemKind::GhastTear => {
                vec![]
            }
            ItemKind::GoldNugget => {
                vec![]
            }
            ItemKind::NetherWart => {
                vec![]
            }
            ItemKind::Potion => {
                vec![]
            }
            ItemKind::GlassBottle => {
                vec![]
            }
            ItemKind::SpiderEye => {
                vec![]
            }
            ItemKind::FermentedSpiderEye => {
                vec![]
            }
            ItemKind::BlazePowder => {
                vec![]
            }
            ItemKind::MagmaCream => {
                vec![]
            }
            ItemKind::BrewingStand => {
                vec![]
            }
            ItemKind::Cauldron => {
                vec![]
            }
            ItemKind::EnderEye => {
                vec![]
            }
            ItemKind::GlisteringMelonSlice => {
                vec![]
            }
            ItemKind::AxolotlSpawnEgg => {
                vec![]
            }
            ItemKind::BatSpawnEgg => {
                vec![]
            }
            ItemKind::BeeSpawnEgg => {
                vec![]
            }
            ItemKind::BlazeSpawnEgg => {
                vec![]
            }
            ItemKind::CatSpawnEgg => {
                vec![]
            }
            ItemKind::CaveSpiderSpawnEgg => {
                vec![]
            }
            ItemKind::ChickenSpawnEgg => {
                vec![]
            }
            ItemKind::CodSpawnEgg => {
                vec![]
            }
            ItemKind::CowSpawnEgg => {
                vec![]
            }
            ItemKind::CreeperSpawnEgg => {
                vec![]
            }
            ItemKind::DolphinSpawnEgg => {
                vec![]
            }
            ItemKind::DonkeySpawnEgg => {
                vec![]
            }
            ItemKind::DrownedSpawnEgg => {
                vec![]
            }
            ItemKind::ElderGuardianSpawnEgg => {
                vec![]
            }
            ItemKind::EndermanSpawnEgg => {
                vec![]
            }
            ItemKind::EndermiteSpawnEgg => {
                vec![]
            }
            ItemKind::EvokerSpawnEgg => {
                vec![]
            }
            ItemKind::FoxSpawnEgg => {
                vec![]
            }
            ItemKind::GhastSpawnEgg => {
                vec![]
            }
            ItemKind::GlowSquidSpawnEgg => {
                vec![]
            }
            ItemKind::GoatSpawnEgg => {
                vec![]
            }
            ItemKind::GuardianSpawnEgg => {
                vec![]
            }
            ItemKind::HoglinSpawnEgg => {
                vec![]
            }
            ItemKind::HorseSpawnEgg => {
                vec![]
            }
            ItemKind::HuskSpawnEgg => {
                vec![]
            }
            ItemKind::LlamaSpawnEgg => {
                vec![]
            }
            ItemKind::MagmaCubeSpawnEgg => {
                vec![]
            }
            ItemKind::MooshroomSpawnEgg => {
                vec![]
            }
            ItemKind::MuleSpawnEgg => {
                vec![]
            }
            ItemKind::OcelotSpawnEgg => {
                vec![]
            }
            ItemKind::PandaSpawnEgg => {
                vec![]
            }
            ItemKind::ParrotSpawnEgg => {
                vec![]
            }
            ItemKind::PhantomSpawnEgg => {
                vec![]
            }
            ItemKind::PigSpawnEgg => {
                vec![]
            }
            ItemKind::PiglinSpawnEgg => {
                vec![]
            }
            ItemKind::PiglinBruteSpawnEgg => {
                vec![]
            }
            ItemKind::PillagerSpawnEgg => {
                vec![]
            }
            ItemKind::PolarBearSpawnEgg => {
                vec![]
            }
            ItemKind::PufferfishSpawnEgg => {
                vec![]
            }
            ItemKind::RabbitSpawnEgg => {
                vec![]
            }
            ItemKind::RavagerSpawnEgg => {
                vec![]
            }
            ItemKind::SalmonSpawnEgg => {
                vec![]
            }
            ItemKind::SheepSpawnEgg => {
                vec![]
            }
            ItemKind::ShulkerSpawnEgg => {
                vec![]
            }
            ItemKind::SilverfishSpawnEgg => {
                vec![]
            }
            ItemKind::SkeletonSpawnEgg => {
                vec![]
            }
            ItemKind::SkeletonHorseSpawnEgg => {
                vec![]
            }
            ItemKind::SlimeSpawnEgg => {
                vec![]
            }
            ItemKind::SpiderSpawnEgg => {
                vec![]
            }
            ItemKind::SquidSpawnEgg => {
                vec![]
            }
            ItemKind::StraySpawnEgg => {
                vec![]
            }
            ItemKind::StriderSpawnEgg => {
                vec![]
            }
            ItemKind::TraderLlamaSpawnEgg => {
                vec![]
            }
            ItemKind::TropicalFishSpawnEgg => {
                vec![]
            }
            ItemKind::TurtleSpawnEgg => {
                vec![]
            }
            ItemKind::VexSpawnEgg => {
                vec![]
            }
            ItemKind::VillagerSpawnEgg => {
                vec![]
            }
            ItemKind::VindicatorSpawnEgg => {
                vec![]
            }
            ItemKind::WanderingTraderSpawnEgg => {
                vec![]
            }
            ItemKind::WitchSpawnEgg => {
                vec![]
            }
            ItemKind::WitherSkeletonSpawnEgg => {
                vec![]
            }
            ItemKind::WolfSpawnEgg => {
                vec![]
            }
            ItemKind::ZoglinSpawnEgg => {
                vec![]
            }
            ItemKind::ZombieSpawnEgg => {
                vec![]
            }
            ItemKind::ZombieHorseSpawnEgg => {
                vec![]
            }
            ItemKind::ZombieVillagerSpawnEgg => {
                vec![]
            }
            ItemKind::ZombifiedPiglinSpawnEgg => {
                vec![]
            }
            ItemKind::ExperienceBottle => {
                vec![]
            }
            ItemKind::FireCharge => {
                vec![]
            }
            ItemKind::WritableBook => {
                vec![]
            }
            ItemKind::WrittenBook => {
                vec![]
            }
            ItemKind::ItemFrame => {
                vec![]
            }
            ItemKind::GlowItemFrame => {
                vec![]
            }
            ItemKind::FlowerPot => {
                vec![]
            }
            ItemKind::Carrot => {
                vec![]
            }
            ItemKind::Potato => {
                vec![]
            }
            ItemKind::BakedPotato => {
                vec![]
            }
            ItemKind::PoisonousPotato => {
                vec![]
            }
            ItemKind::Map => {
                vec![]
            }
            ItemKind::GoldenCarrot => {
                vec![]
            }
            ItemKind::SkeletonSkull => {
                vec![]
            }
            ItemKind::WitherSkeletonSkull => {
                vec![]
            }
            ItemKind::PlayerHead => {
                vec![]
            }
            ItemKind::ZombieHead => {
                vec![]
            }
            ItemKind::CreeperHead => {
                vec![]
            }
            ItemKind::DragonHead => {
                vec![]
            }
            ItemKind::NetherStar => {
                vec![]
            }
            ItemKind::PumpkinPie => {
                vec![]
            }
            ItemKind::FireworkRocket => {
                vec![]
            }
            ItemKind::FireworkStar => {
                vec![]
            }
            ItemKind::EnchantedBook => {
                vec![]
            }
            ItemKind::NetherBrick => {
                vec![]
            }
            ItemKind::PrismarineShard => {
                vec![]
            }
            ItemKind::PrismarineCrystals => {
                vec![]
            }
            ItemKind::Rabbit => {
                vec![]
            }
            ItemKind::CookedRabbit => {
                vec![]
            }
            ItemKind::RabbitStew => {
                vec![]
            }
            ItemKind::RabbitFoot => {
                vec![]
            }
            ItemKind::RabbitHide => {
                vec![]
            }
            ItemKind::ArmorStand => {
                vec![]
            }
            ItemKind::IronHorseArmor => {
                vec![]
            }
            ItemKind::GoldenHorseArmor => {
                vec![]
            }
            ItemKind::DiamondHorseArmor => {
                vec![]
            }
            ItemKind::LeatherHorseArmor => {
                vec![]
            }
            ItemKind::Lead => {
                vec![]
            }
            ItemKind::NameTag => {
                vec![]
            }
            ItemKind::CommandBlockMinecart => {
                vec![]
            }
            ItemKind::Mutton => {
                vec![]
            }
            ItemKind::CookedMutton => {
                vec![]
            }
            ItemKind::WhiteBanner => {
                vec![]
            }
            ItemKind::OrangeBanner => {
                vec![]
            }
            ItemKind::MagentaBanner => {
                vec![]
            }
            ItemKind::LightBlueBanner => {
                vec![]
            }
            ItemKind::YellowBanner => {
                vec![]
            }
            ItemKind::LimeBanner => {
                vec![]
            }
            ItemKind::PinkBanner => {
                vec![]
            }
            ItemKind::GrayBanner => {
                vec![]
            }
            ItemKind::LightGrayBanner => {
                vec![]
            }
            ItemKind::CyanBanner => {
                vec![]
            }
            ItemKind::PurpleBanner => {
                vec![]
            }
            ItemKind::BlueBanner => {
                vec![]
            }
            ItemKind::BrownBanner => {
                vec![]
            }
            ItemKind::GreenBanner => {
                vec![]
            }
            ItemKind::RedBanner => {
                vec![]
            }
            ItemKind::BlackBanner => {
                vec![]
            }
            ItemKind::EndCrystal => {
                vec![]
            }
            ItemKind::ChorusFruit => {
                vec![]
            }
            ItemKind::PoppedChorusFruit => {
                vec![]
            }
            ItemKind::Beetroot => {
                vec![]
            }
            ItemKind::BeetrootSeeds => {
                vec![]
            }
            ItemKind::BeetrootSoup => {
                vec![]
            }
            ItemKind::DragonBreath => {
                vec![]
            }
            ItemKind::SplashPotion => {
                vec![]
            }
            ItemKind::SpectralArrow => {
                vec![]
            }
            ItemKind::TippedArrow => {
                vec![]
            }
            ItemKind::LingeringPotion => {
                vec![]
            }
            ItemKind::Shield => {
                vec![]
            }
            ItemKind::TotemOfUndying => {
                vec![]
            }
            ItemKind::ShulkerShell => {
                vec![]
            }
            ItemKind::IronNugget => {
                vec![]
            }
            ItemKind::KnowledgeBook => {
                vec![]
            }
            ItemKind::DebugStick => {
                vec![]
            }
            ItemKind::MusicDisc13 => {
                vec![]
            }
            ItemKind::MusicDiscCat => {
                vec![]
            }
            ItemKind::MusicDiscBlocks => {
                vec![]
            }
            ItemKind::MusicDiscChirp => {
                vec![]
            }
            ItemKind::MusicDiscFar => {
                vec![]
            }
            ItemKind::MusicDiscMall => {
                vec![]
            }
            ItemKind::MusicDiscMellohi => {
                vec![]
            }
            ItemKind::MusicDiscStal => {
                vec![]
            }
            ItemKind::MusicDiscStrad => {
                vec![]
            }
            ItemKind::MusicDiscWard => {
                vec![]
            }
            ItemKind::MusicDisc11 => {
                vec![]
            }
            ItemKind::MusicDiscWait => {
                vec![]
            }
            ItemKind::MusicDiscOtherside => {
                vec![]
            }
            ItemKind::MusicDiscPigstep => {
                vec![]
            }
            ItemKind::Trident => {
                vec![]
            }
            ItemKind::PhantomMembrane => {
                vec![]
            }
            ItemKind::NautilusShell => {
                vec![]
            }
            ItemKind::HeartOfTheSea => {
                vec![]
            }
            ItemKind::Crossbow => {
                vec![]
            }
            ItemKind::SuspiciousStew => {
                vec![]
            }
            ItemKind::Loom => {
                vec![]
            }
            ItemKind::FlowerBannerPattern => {
                vec![]
            }
            ItemKind::CreeperBannerPattern => {
                vec![]
            }
            ItemKind::SkullBannerPattern => {
                vec![]
            }
            ItemKind::MojangBannerPattern => {
                vec![]
            }
            ItemKind::GlobeBannerPattern => {
                vec![]
            }
            ItemKind::PiglinBannerPattern => {
                vec![]
            }
            ItemKind::Composter => {
                vec![]
            }
            ItemKind::Barrel => {
                vec![]
            }
            ItemKind::Smoker => {
                vec![]
            }
            ItemKind::BlastFurnace => {
                vec![]
            }
            ItemKind::CartographyTable => {
                vec![]
            }
            ItemKind::FletchingTable => {
                vec![]
            }
            ItemKind::Grindstone => {
                vec![]
            }
            ItemKind::SmithingTable => {
                vec![]
            }
            ItemKind::Stonecutter => {
                vec![]
            }
            ItemKind::Bell => {
                vec![]
            }
            ItemKind::Lantern => {
                vec![]
            }
            ItemKind::SoulLantern => {
                vec![]
            }
            ItemKind::SweetBerries => {
                vec![]
            }
            ItemKind::GlowBerries => {
                vec![]
            }
            ItemKind::Campfire => {
                vec![]
            }
            ItemKind::SoulCampfire => {
                vec![]
            }
            ItemKind::Shroomlight => {
                vec![]
            }
            ItemKind::Honeycomb => {
                vec![]
            }
            ItemKind::BeeNest => {
                vec![]
            }
            ItemKind::Beehive => {
                vec![]
            }
            ItemKind::HoneyBottle => {
                vec![]
            }
            ItemKind::HoneycombBlock => {
                vec![]
            }
            ItemKind::Lodestone => {
                vec![]
            }
            ItemKind::CryingObsidian => {
                vec![]
            }
            ItemKind::Blackstone => {
                vec![]
            }
            ItemKind::BlackstoneSlab => {
                vec![]
            }
            ItemKind::BlackstoneStairs => {
                vec![]
            }
            ItemKind::GildedBlackstone => {
                vec![]
            }
            ItemKind::PolishedBlackstone => {
                vec![]
            }
            ItemKind::PolishedBlackstoneSlab => {
                vec![]
            }
            ItemKind::PolishedBlackstoneStairs => {
                vec![]
            }
            ItemKind::ChiseledPolishedBlackstone => {
                vec![]
            }
            ItemKind::PolishedBlackstoneBricks => {
                vec![]
            }
            ItemKind::PolishedBlackstoneBrickSlab => {
                vec![]
            }
            ItemKind::PolishedBlackstoneBrickStairs => {
                vec![]
            }
            ItemKind::CrackedPolishedBlackstoneBricks => {
                vec![]
            }
            ItemKind::RespawnAnchor => {
                vec![]
            }
            ItemKind::Candle => {
                vec![]
            }
            ItemKind::WhiteCandle => {
                vec![]
            }
            ItemKind::OrangeCandle => {
                vec![]
            }
            ItemKind::MagentaCandle => {
                vec![]
            }
            ItemKind::LightBlueCandle => {
                vec![]
            }
            ItemKind::YellowCandle => {
                vec![]
            }
            ItemKind::LimeCandle => {
                vec![]
            }
            ItemKind::PinkCandle => {
                vec![]
            }
            ItemKind::GrayCandle => {
                vec![]
            }
            ItemKind::LightGrayCandle => {
                vec![]
            }
            ItemKind::CyanCandle => {
                vec![]
            }
            ItemKind::PurpleCandle => {
                vec![]
            }
            ItemKind::BlueCandle => {
                vec![]
            }
            ItemKind::BrownCandle => {
                vec![]
            }
            ItemKind::GreenCandle => {
                vec![]
            }
            ItemKind::RedCandle => {
                vec![]
            }
            ItemKind::BlackCandle => {
                vec![]
            }
            ItemKind::SmallAmethystBud => {
                vec![]
            }
            ItemKind::MediumAmethystBud => {
                vec![]
            }
            ItemKind::LargeAmethystBud => {
                vec![]
            }
            ItemKind::AmethystCluster => {
                vec![]
            }
            ItemKind::PointedDripstone => {
                vec![]
            }
        }
    }
}
use std::convert::TryFrom;
use std::str::FromStr;
impl TryFrom<String> for ItemKind {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(item) = ItemKind::from_name(value.as_str()) {
            Ok(item)
        } else {
            Err("Unknown item name")
        }
    }
}
impl From<ItemKind> for &'static str {
    fn from(i: ItemKind) -> Self {
        i.name()
    }
}
impl FromStr for ItemKind {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(item) = ItemKind::from_name(s) {
            Ok(item)
        } else {
            Err("Unknown item name")
        }
    }
}
