//! Color data structures and conversions.
use crate::ParseStructError;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

/// Color color data structure.
#[derive(Clone, Debug)]
pub struct Color {
    channels: [u8; 4],
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            channels: [r, g, b, a],
        }
    }
    pub fn channels(&self) -> &[u8; 4] {
        &self.channels
    }
}

impl FromStr for Color {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if COLORS.contains_key(s) {
            Ok(COLORS[s].clone())
        } else {
            let parts: Vec<_> = s.split("/").collect();

            match parts.len() {
                1 => {
                    let v = parts[0].parse()?;
                    Ok(Color::new(v, v, v, 255))
                }
                3 => Ok(Color::new(
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    255,
                )),
                4 => Ok(Color::new(
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    parts[4].parse()?,
                )),
                _ => Err(Box::new(ParseStructError(format!(
                    "Can't parse color from {}, requires 1, 3 or 4 elements",
                    s
                )))),
            }
        }
    }
}

lazy_static! {
    /// Named color.
    ///
    /// ## Available color names:
    /// <pre>
    /// air_force_blue_raf
    /// air_force_blue_usaf
    /// air_superiority_blue
    /// alabama_crimson
    /// alice_blue
    /// alizarin_crimson
    /// alloy_orange
    /// almond
    /// amaranth
    /// amber
    /// amber_sae_ece
    /// american_rose
    /// amethyst
    /// android_green
    /// anti_flash_white
    /// antique_brass
    /// antique_fuchsia
    /// antique_ruby
    /// antique_white
    /// ao_english
    /// apple_green
    /// apricot
    /// aqua
    /// aquamarine
    /// army_green
    /// arsenic
    /// arylide_yellow
    /// ash_grey
    /// asparagus
    /// atomic_tangerine
    /// auburn
    /// aureolin
    /// aurometalsaurus
    /// avocado
    /// azure
    /// azure_mist_web
    /// baby_blue
    /// baby_blue_eyes
    /// baby_pink
    /// ball_blue
    /// banana_mania
    /// banana_yellow
    /// barn_red
    /// battleship_grey
    /// bazaar
    /// beau_blue
    /// beaver
    /// beige
    /// big_dip_o_ruby
    /// bisque
    /// bistre
    /// bittersweet
    /// bittersweet_shimmer
    /// black
    /// black_bean
    /// black_leather_jacket
    /// black_olive
    /// blanched_almond
    /// blast_off_bronze
    /// bleu_de_france
    /// blizzard_blue
    /// blond
    /// blue
    /// blue_bell
    /// blue_crayola
    /// blue_gray
    /// blue_green
    /// blue_munsell
    /// blue_ncs
    /// blue_pigment
    /// blue_ryb
    /// blue_sapphire
    /// blue_violet
    /// blush
    /// bole
    /// bondi_blue
    /// bone
    /// boston_university_red
    /// bottle_green
    /// boysenberry
    /// brandeis_blue
    /// brass
    /// brick_red
    /// bright_cerulean
    /// bright_green
    /// bright_lavender
    /// bright_maroon
    /// bright_pink
    /// bright_turquoise
    /// bright_ube
    /// brilliant_lavender
    /// brilliant_rose
    /// brink_pink
    /// british_racing_green
    /// bronze
    /// brown_traditional
    /// brown_web
    /// bubble_gum
    /// bubbles
    /// buff
    /// bulgarian_rose
    /// burgundy
    /// burlywood
    /// burnt_orange
    /// burnt_sienna
    /// burnt_umber
    /// byzantine
    /// byzantium
    /// cadet
    /// cadet_blue
    /// cadet_grey
    /// cadmium_green
    /// cadmium_orange
    /// cadmium_red
    /// cadmium_yellow
    /// caf_au_lait
    /// caf_noir
    /// cal_poly_green
    /// cambridge_blue
    /// camel
    /// cameo_pink
    /// camouflage_green
    /// canary_yellow
    /// candy_apple_red
    /// candy_pink
    /// capri
    /// caput_mortuum
    /// cardinal
    /// caribbean_green
    /// carmine
    /// carmine_m_p
    /// carmine_pink
    /// carmine_red
    /// carnation_pink
    /// carnelian
    /// carolina_blue
    /// carrot_orange
    /// catalina_blue
    /// ceil
    /// celadon
    /// celadon_blue
    /// celadon_green
    /// celeste_colour
    /// celestial_blue
    /// cerise
    /// cerise_pink
    /// cerulean
    /// cerulean_blue
    /// cerulean_frost
    /// cg_blue
    /// cg_red
    /// chamoisee
    /// champagne
    /// charcoal
    /// charm_pink
    /// chartreuse_traditional
    /// chartreuse_web
    /// cherry
    /// cherry_blossom_pink
    /// chestnut
    /// china_pink
    /// china_rose
    /// chinese_red
    /// chocolate_traditional
    /// chocolate_web
    /// chrome_yellow
    /// cinereous
    /// cinnabar
    /// cinnamon
    /// citrine
    /// classic_rose
    /// cobalt
    /// cocoa_brown
    /// coffee
    /// columbia_blue
    /// congo_pink
    /// cool_black
    /// cool_grey
    /// copper
    /// copper_crayola
    /// copper_penny
    /// copper_red
    /// copper_rose
    /// coquelicot
    /// coral
    /// coral_pink
    /// coral_red
    /// cordovan
    /// corn
    /// cornell_red
    /// cornflower_blue
    /// cornsilk
    /// cosmic_latte
    /// cotton_candy
    /// cream
    /// crimson
    /// crimson_glory
    /// cyan
    /// cyan_process
    /// daffodil
    /// dandelion
    /// dark_blue
    /// dark_brown
    /// dark_byzantium
    /// dark_candy_apple_red
    /// dark_cerulean
    /// dark_chestnut
    /// dark_coral
    /// dark_cyan
    /// dark_electric_blue
    /// dark_goldenrod
    /// dark_gray
    /// dark_green
    /// dark_imperial_blue
    /// dark_jungle_green
    /// dark_khaki
    /// dark_lava
    /// dark_lavender
    /// dark_magenta
    /// dark_midnight_blue
    /// dark_olive_green
    /// dark_orange
    /// dark_orchid
    /// dark_pastel_blue
    /// dark_pastel_green
    /// dark_pastel_purple
    /// dark_pastel_red
    /// dark_pink
    /// dark_powder_blue
    /// dark_raspberry
    /// dark_red
    /// dark_salmon
    /// dark_scarlet
    /// dark_sea_green
    /// dark_sienna
    /// dark_slate_blue
    /// dark_slate_gray
    /// dark_spring_green
    /// dark_tan
    /// dark_tangerine
    /// dark_taupe
    /// dark_terra_cotta
    /// dark_turquoise
    /// dark_violet
    /// dark_yellow
    /// dartmouth_green
    /// davy_s_grey
    /// debian_red
    /// deep_carmine
    /// deep_carmine_pink
    /// deep_carrot_orange
    /// deep_cerise
    /// deep_champagne
    /// deep_chestnut
    /// deep_coffee
    /// deep_fuchsia
    /// deep_jungle_green
    /// deep_lilac
    /// deep_magenta
    /// deep_peach
    /// deep_pink
    /// deep_ruby
    /// deep_saffron
    /// deep_sky_blue
    /// deep_tuscan_red
    /// denim
    /// desert
    /// desert_sand
    /// dim_gray
    /// dodger_blue
    /// dogwood_rose
    /// dollar_bill
    /// drab
    /// duke_blue
    /// earth_yellow
    /// ebony
    /// ecru
    /// eggplant
    /// eggshell
    /// egyptian_blue
    /// electric_blue
    /// electric_crimson
    /// electric_cyan
    /// electric_green
    /// electric_indigo
    /// electric_lavender
    /// electric_lime
    /// electric_purple
    /// electric_ultramarine
    /// electric_violet
    /// electric_yellow
    /// emerald
    /// english_lavender
    /// eton_blue
    /// fallow
    /// falu_red
    /// fandango
    /// fashion_fuchsia
    /// fawn
    /// feldgrau
    /// fern_green
    /// ferrari_red
    /// field_drab
    /// fire_engine_red
    /// firebrick
    /// flame
    /// flamingo_pink
    /// flavescent
    /// flax
    /// floral_white
    /// fluorescent_orange
    /// fluorescent_pink
    /// fluorescent_yellow
    /// folly
    /// forest_green_traditional
    /// forest_green_web
    /// french_beige
    /// french_blue
    /// french_lilac
    /// french_lime
    /// french_raspberry
    /// french_rose
    /// fuchsia
    /// fuchsia_crayola
    /// fuchsia_pink
    /// fuchsia_rose
    /// fulvous
    /// fuzzy_wuzzy
    /// gainsboro
    /// gamboge
    /// ghost_white
    /// ginger
    /// glaucous
    /// glitter
    /// gold_metallic
    /// gold_web_golden
    /// golden_brown
    /// golden_poppy
    /// golden_yellow
    /// goldenrod
    /// granny_smith_apple
    /// gray
    /// gray_asparagus
    /// gray_html_css_gray
    /// gray_x11_gray
    /// green_color_wheel_x11_green
    /// green_crayola
    /// green_html_css_green
    /// green_munsell
    /// green_ncs
    /// green_pigment
    /// green_ryb
    /// green_yellow
    /// grullo
    /// guppie_green
    /// halay_be
    /// han_blue
    /// han_purple
    /// hansa_yellow
    /// harlequin
    /// harvard_crimson
    /// harvest_gold
    /// heart_gold
    /// heliotrope
    /// hollywood_cerise
    /// honeydew
    /// honolulu_blue
    /// hooker_s_green
    /// hot_magenta
    /// hot_pink
    /// hunter_green
    /// iceberg
    /// icterine
    /// imperial_blue
    /// inchworm
    /// india_green
    /// indian_red
    /// indian_yellow
    /// indigo
    /// indigo_dye
    /// indigo_web
    /// international_klein_blue
    /// international_orange_aerospace
    /// international_orange_engineering
    /// international_orange_golden_gate_bridge
    /// iris
    /// isabelline
    /// islamic_green
    /// ivory
    /// jade
    /// jasmine
    /// jasper
    /// jazzberry_jam
    /// jet
    /// jonquil
    /// june_bud
    /// jungle_green
    /// kelly_green
    /// kenyan_copper
    /// khaki_html_css_khaki
    /// khaki_x11_light_khaki
    /// ku_crimson
    /// la_salle_green
    /// languid_lavender
    /// lapis_lazuli
    /// laser_lemon
    /// laurel_green
    /// lava
    /// lavender_blue
    /// lavender_blush
    /// lavender_floral
    /// lavender_gray
    /// lavender_indigo
    /// lavender_magenta
    /// lavender_mist
    /// lavender_pink
    /// lavender_purple
    /// lavender_rose
    /// lavender_web
    /// lawn_green
    /// lemon
    /// lemon_chiffon
    /// lemon_lime
    /// licorice
    /// light_apricot
    /// light_blue
    /// light_brown
    /// light_carmine_pink
    /// light_coral
    /// light_cornflower_blue
    /// light_crimson
    /// light_cyan
    /// light_fuchsia_pink
    /// light_goldenrod_yellow
    /// light_gray
    /// light_green
    /// light_khaki
    /// light_pastel_purple
    /// light_pink
    /// light_red_ochre
    /// light_salmon
    /// light_salmon_pink
    /// light_sea_green
    /// light_sky_blue
    /// light_slate_gray
    /// light_taupe
    /// light_thulian_pink
    /// light_yellow
    /// lilac
    /// lime_color_wheel
    /// lime_green
    /// lime_web_x11_green
    /// limerick
    /// lincoln_green
    /// linen
    /// lion
    /// little_boy_blue
    /// liver
    /// lust
    /// magenta
    /// magenta_dye
    /// magenta_process
    /// magic_mint
    /// magnolia
    /// mahogany
    /// maize
    /// majorelle_blue
    /// malachite
    /// manatee
    /// mango_tango
    /// mantis
    /// mardi_gras
    /// maroon_crayola
    /// maroon_html_css
    /// maroon_x11
    /// mauve
    /// mauve_taupe
    /// mauvelous
    /// maya_blue
    /// meat_brown
    /// medium_aquamarine
    /// medium_blue
    /// medium_candy_apple_red
    /// medium_carmine
    /// medium_champagne
    /// medium_electric_blue
    /// medium_jungle_green
    /// medium_lavender_magenta
    /// medium_orchid
    /// medium_persian_blue
    /// medium_purple
    /// medium_red_violet
    /// medium_ruby
    /// medium_sea_green
    /// medium_slate_blue
    /// medium_spring_bud
    /// medium_spring_green
    /// medium_taupe
    /// medium_turquoise
    /// medium_tuscan_red
    /// medium_vermilion
    /// medium_violet_red
    /// mellow_apricot
    /// mellow_yellow
    /// melon
    /// midnight_blue
    /// midnight_green_eagle_green
    /// mikado_yellow
    /// mint
    /// mint_cream
    /// mint_green
    /// misty_rose
    /// moccasin
    /// mode_beige
    /// moonstone_blue
    /// mordant_red_19
    /// moss_green
    /// mountain_meadow
    /// mountbatten_pink
    /// msu_green
    /// mulberry
    /// mustard
    /// myrtle
    /// nadeshiko_pink
    /// napier_green
    /// naples_yellow
    /// navajo_white
    /// navy_blue
    /// neon_carrot
    /// neon_fuchsia
    /// neon_green
    /// new_york_pink
    /// non_photo_blue
    /// north_texas_green
    /// ocean_boat_blue
    /// ochre
    /// office_green
    /// old_gold
    /// old_lace
    /// old_lavender
    /// old_mauve
    /// old_rose
    /// olive
    /// olive_drab_7
    /// olive_drab_web_olive_drab_3
    /// olivine
    /// onyx
    /// opera_mauve
    /// orange_color_wheel
    /// orange_peel
    /// orange_red
    /// orange_ryb
    /// orange_web_color
    /// orchid
    /// otter_brown
    /// ou_crimson_red
    /// outer_space
    /// outrageous_orange
    /// oxford_blue
    /// pakistan_green
    /// palatinate_blue
    /// palatinate_purple
    /// pale_aqua
    /// pale_blue
    /// pale_brown
    /// pale_carmine
    /// pale_cerulean
    /// pale_chestnut
    /// pale_copper
    /// pale_cornflower_blue
    /// pale_gold
    /// pale_goldenrod
    /// pale_green
    /// pale_lavender
    /// pale_magenta
    /// pale_pink
    /// pale_plum
    /// pale_red_violet
    /// pale_robin_egg_blue
    /// pale_silver
    /// pale_spring_bud
    /// pale_taupe
    /// pale_violet_red
    /// pansy_purple
    /// papaya_whip
    /// paris_green
    /// pastel_blue
    /// pastel_brown
    /// pastel_gray
    /// pastel_green
    /// pastel_magenta
    /// pastel_orange
    /// pastel_pink
    /// pastel_purple
    /// pastel_red
    /// pastel_violet
    /// pastel_yellow
    /// patriarch
    /// payne_s_grey
    /// peach
    /// peach_crayola
    /// peach_orange
    /// peach_puff
    /// peach_yellow
    /// pear
    /// pearl
    /// pearl_aqua
    /// pearly_purple
    /// peridot
    /// periwinkle
    /// persian_blue
    /// persian_green
    /// persian_indigo
    /// persian_orange
    /// persian_pink
    /// persian_plum
    /// persian_red
    /// persian_rose
    /// persimmon
    /// peru
    /// phlox
    /// phthalo_blue
    /// phthalo_green
    /// piggy_pink
    /// pine_green
    /// pink
    /// pink_lace
    /// pink_orange
    /// pink_pearl
    /// pink_sherbet
    /// pistachio
    /// platinum
    /// plum_traditional
    /// plum_web
    /// portland_orange
    /// powder_blue_web
    /// princeton_orange
    /// prune
    /// prussian_blue
    /// psychedelic_purple
    /// puce
    /// pumpkin
    /// purple_heart
    /// purple_html_css
    /// purple_mountain_majesty
    /// purple_munsell
    /// purple_pizzazz
    /// purple_taupe
    /// purple_x11
    /// quartz
    /// rackley
    /// radical_red
    /// rajah
    /// raspberry
    /// raspberry_glace
    /// raspberry_pink
    /// raspberry_rose
    /// raw_umber
    /// razzle_dazzle_rose
    /// razzmatazz
    /// red
    /// red_brown
    /// red_devil
    /// red_munsell
    /// red_ncs
    /// red_orange
    /// red_pigment
    /// red_ryb
    /// red_violet
    /// redwood
    /// regalia
    /// resolution_blue
    /// rich_black
    /// rich_brilliant_lavender
    /// rich_carmine
    /// rich_electric_blue
    /// rich_lavender
    /// rich_lilac
    /// rich_maroon
    /// rifle_green
    /// robin_egg_blue
    /// rose
    /// rose_bonbon
    /// rose_ebony
    /// rose_gold
    /// rose_madder
    /// rose_pink
    /// rose_quartz
    /// rose_taupe
    /// rose_vale
    /// rosewood
    /// rosso_corsa
    /// rosy_brown
    /// royal_azure
    /// royal_blue_traditional
    /// royal_blue_web
    /// royal_fuchsia
    /// royal_purple
    /// royal_yellow
    /// rubine_red
    /// ruby
    /// ruby_red
    /// ruddy
    /// ruddy_brown
    /// ruddy_pink
    /// rufous
    /// russet
    /// rust
    /// rusty_red
    /// sacramento_state_green
    /// saddle_brown
    /// safety_orange_blaze_orange
    /// saffron
    /// salmon
    /// salmon_pink
    /// sand
    /// sand_dune
    /// sandstorm
    /// sandy_brown
    /// sandy_taupe
    /// sangria
    /// sap_green
    /// sapphire
    /// sapphire_blue
    /// satin_sheen_gold
    /// scarlet
    /// scarlet_crayola
    /// school_bus_yellow
    /// screamin_green
    /// sea_blue
    /// sea_green
    /// seal_brown
    /// seashell
    /// selective_yellow
    /// sepia
    /// shadow
    /// shamrock_green
    /// shocking_pink
    /// shocking_pink_crayola
    /// sienna
    /// silver
    /// sinopia
    /// skobeloff
    /// sky_blue
    /// sky_magenta
    /// slate_blue
    /// slate_gray
    /// smalt_dark_powder_blue
    /// smokey_topaz
    /// smoky_black
    /// snow
    /// spiro_disco_ball
    /// spring_bud
    /// spring_green
    /// st_patrick_s_blue
    /// steel_blue
    /// stil_de_grain_yellow
    /// stizza
    /// stormcloud
    /// straw
    /// sunglow
    /// sunset
    /// tan
    /// tangelo
    /// tangerine
    /// tangerine_yellow
    /// tango_pink
    /// taupe
    /// taupe_gray
    /// tea_green
    /// tea_rose_orange
    /// tea_rose_rose
    /// teal
    /// teal_blue
    /// teal_green
    /// telemagenta
    /// tenn_tawny
    /// terra_cotta
    /// thistle
    /// thulian_pink
    /// tickle_me_pink
    /// tiffany_blue
    /// tiger_s_eye
    /// timberwolf
    /// titanium_yellow
    /// tomato
    /// toolbox
    /// topaz
    /// tractor_red
    /// trolley_grey
    /// tropical_rain_forest
    /// true_blue
    /// tufts_blue
    /// tumbleweed
    /// turkish_rose
    /// turquoise
    /// turquoise_blue
    /// turquoise_green
    /// tuscan_red
    /// twilight_lavender
    /// tyrian_purple
    /// ua_blue
    /// ua_red
    /// ube
    /// ucla_blue
    /// ucla_gold
    /// ufo_green
    /// ultra_pink
    /// ultramarine
    /// ultramarine_blue
    /// umber
    /// unbleached_silk
    /// united_nations_blue
    /// university_of_california_gold
    /// unmellow_yellow
    /// up_forest_green
    /// up_maroon
    /// upsdell_red
    /// urobilin
    /// usafa_blue
    /// usc_cardinal
    /// usc_gold
    /// utah_crimson
    /// vanilla
    /// vegas_gold
    /// venetian_red
    /// verdigris
    /// vermilion_cinnabar
    /// vermilion_plochere
    /// veronica
    /// violet
    /// violet_blue
    /// violet_color_wheel
    /// violet_ryb
    /// violet_web
    /// viridian
    /// vivid_auburn
    /// vivid_burgundy
    /// vivid_cerise
    /// vivid_tangerine
    /// vivid_violet
    /// warm_black
    /// waterspout
    /// wenge
    /// wheat
    /// white
    /// white_smoke
    /// wild_blue_yonder
    /// wild_strawberry
    /// wild_watermelon
    /// wine
    /// wine_dregs
    /// wisteria
    /// wood_brown
    /// xanadu
    /// yale_blue
    /// yellow
    /// yellow_green
    /// yellow_munsell
    /// yellow_ncs
    /// yellow_orange
    /// yellow_process
    /// yellow_ryb
    /// zaffre
    /// zinnwaldite_brown
    /// </pre>
    pub static ref COLORS: HashMap<&'static str, Color> = {
        let mut m = HashMap::new();
        m.insert("transparent", Color::new(255, 255, 255, 0));
        m.insert("air_force_blue_raf", Color::new(93, 138, 168, 255));
        m.insert("air_force_blue_usaf", Color::new(0, 48, 143, 255));
        m.insert("air_superiority_blue", Color::new(114, 160, 193, 255));
        m.insert("alabama_crimson", Color::new(163, 38, 56, 255));
        m.insert("alice_blue", Color::new(240, 248, 255, 255));
        m.insert("alizarin_crimson", Color::new(227, 38, 54, 255));
        m.insert("alloy_orange", Color::new(196, 98, 16, 255));
        m.insert("almond", Color::new(239, 222, 205, 255));
        m.insert("amaranth", Color::new(229, 43, 80, 255));
        m.insert("amber", Color::new(255, 191, 0, 255));
        m.insert("amber_sae_ece", Color::new(255, 126, 0, 255));
        m.insert("american_rose", Color::new(255, 3, 62, 255));
        m.insert("amethyst", Color::new(153, 102, 204, 255));
        m.insert("android_green", Color::new(164, 198, 57, 255));
        m.insert("anti_flash_white", Color::new(242, 243, 244, 255));
        m.insert("antique_brass", Color::new(205, 149, 117, 255));
        m.insert("antique_fuchsia", Color::new(145, 92, 131, 255));
        m.insert("antique_ruby", Color::new(132, 27, 45, 255));
        m.insert("antique_white", Color::new(250, 235, 215, 255));
        m.insert("ao_english", Color::new(0, 128, 0, 255));
        m.insert("apple_green", Color::new(141, 182, 0, 255));
        m.insert("apricot", Color::new(251, 206, 177, 255));
        m.insert("aqua", Color::new(0, 255, 255, 255));
        m.insert("aquamarine", Color::new(127, 255, 212, 255));
        m.insert("army_green", Color::new(75, 83, 32, 255));
        m.insert("arsenic", Color::new(59, 68, 75, 255));
        m.insert("arylide_yellow", Color::new(233, 214, 107, 255));
        m.insert("ash_grey", Color::new(178, 190, 181, 255));
        m.insert("asparagus", Color::new(135, 169, 107, 255));
        m.insert("atomic_tangerine", Color::new(255, 153, 102, 255));
        m.insert("auburn", Color::new(165, 42, 42, 255));
        m.insert("aureolin", Color::new(253, 238, 0, 255));
        m.insert("aurometalsaurus", Color::new(110, 127, 128, 255));
        m.insert("avocado", Color::new(86, 130, 3, 255));
        m.insert("azure", Color::new(0, 127, 255, 255));
        m.insert("azure_mist_web", Color::new(240, 255, 255, 255));
        m.insert("baby_blue", Color::new(137, 207, 240, 255));
        m.insert("baby_blue_eyes", Color::new(161, 202, 241, 255));
        m.insert("baby_pink", Color::new(244, 194, 194, 255));
        m.insert("ball_blue", Color::new(33, 171, 205, 255));
        m.insert("banana_mania", Color::new(250, 231, 181, 255));
        m.insert("banana_yellow", Color::new(255, 225, 53, 255));
        m.insert("barn_red", Color::new(124, 10, 2, 255));
        m.insert("battleship_grey", Color::new(132, 132, 130, 255));
        m.insert("bazaar", Color::new(152, 119, 123, 255));
        m.insert("beau_blue", Color::new(188, 212, 230, 255));
        m.insert("beaver", Color::new(159, 129, 112, 255));
        m.insert("beige", Color::new(245, 245, 220, 255));
        m.insert("big_dip_o_ruby", Color::new(156, 37, 66, 255));
        m.insert("bisque", Color::new(255, 228, 196, 255));
        m.insert("bistre", Color::new(61, 43, 31, 255));
        m.insert("bittersweet", Color::new(254, 111, 94, 255));
        m.insert("bittersweet_shimmer", Color::new(191, 79, 81, 255));
        m.insert("black", Color::new(0, 0, 0, 255));
        m.insert("black_bean", Color::new(61, 12, 2, 255));
        m.insert("black_leather_jacket", Color::new(37, 53, 41, 255));
        m.insert("black_olive", Color::new(59, 60, 54, 255));
        m.insert("blanched_almond", Color::new(255, 235, 205, 255));
        m.insert("blast_off_bronze", Color::new(165, 113, 100, 255));
        m.insert("bleu_de_france", Color::new(49, 140, 231, 255));
        m.insert("blizzard_blue", Color::new(172, 229, 238, 255));
        m.insert("blond", Color::new(250, 240, 190, 255));
        m.insert("blue", Color::new(0, 0, 255, 255));
        m.insert("blue_bell", Color::new(162, 162, 208, 255));
        m.insert("blue_crayola", Color::new(31, 117, 254, 255));
        m.insert("blue_gray", Color::new(102, 153, 204, 255));
        m.insert("blue_green", Color::new(13, 152, 186, 255));
        m.insert("blue_munsell", Color::new(0, 147, 175, 255));
        m.insert("blue_ncs", Color::new(0, 135, 189, 255));
        m.insert("blue_pigment", Color::new(51, 51, 153, 255));
        m.insert("blue_ryb", Color::new(2, 71, 254, 255));
        m.insert("blue_sapphire", Color::new(18, 97, 128, 255));
        m.insert("blue_violet", Color::new(138, 43, 226, 255));
        m.insert("blush", Color::new(222, 93, 131, 255));
        m.insert("bole", Color::new(121, 68, 59, 255));
        m.insert("bondi_blue", Color::new(0, 149, 182, 255));
        m.insert("bone", Color::new(227, 218, 201, 255));
        m.insert("boston_university_red", Color::new(204, 0, 0, 255));
        m.insert("bottle_green", Color::new(0, 106, 78, 255));
        m.insert("boysenberry", Color::new(135, 50, 96, 255));
        m.insert("brandeis_blue", Color::new(0, 112, 255, 255));
        m.insert("brass", Color::new(181, 166, 66, 255));
        m.insert("brick_red", Color::new(203, 65, 84, 255));
        m.insert("bright_cerulean", Color::new(29, 172, 214, 255));
        m.insert("bright_green", Color::new(102, 255, 0, 255));
        m.insert("bright_lavender", Color::new(191, 148, 228, 255));
        m.insert("bright_maroon", Color::new(195, 33, 72, 255));
        m.insert("bright_pink", Color::new(255, 0, 127, 255));
        m.insert("bright_turquoise", Color::new(8, 232, 222, 255));
        m.insert("bright_ube", Color::new(209, 159, 232, 255));
        m.insert("brilliant_lavender", Color::new(244, 187, 255, 255));
        m.insert("brilliant_rose", Color::new(255, 85, 163, 255));
        m.insert("brink_pink", Color::new(251, 96, 127, 255));
        m.insert("british_racing_green", Color::new(0, 66, 37, 255));
        m.insert("bronze", Color::new(205, 127, 50, 255));
        m.insert("brown_traditional", Color::new(150, 75, 0, 255));
        m.insert("brown_web", Color::new(165, 42, 42, 255));
        m.insert("bubble_gum", Color::new(255, 193, 204, 255));
        m.insert("bubbles", Color::new(231, 254, 255, 255));
        m.insert("buff", Color::new(240, 220, 130, 255));
        m.insert("bulgarian_rose", Color::new(72, 6, 7, 255));
        m.insert("burgundy", Color::new(128, 0, 32, 255));
        m.insert("burlywood", Color::new(222, 184, 135, 255));
        m.insert("burnt_orange", Color::new(204, 85, 0, 255));
        m.insert("burnt_sienna", Color::new(233, 116, 81, 255));
        m.insert("burnt_umber", Color::new(138, 51, 36, 255));
        m.insert("byzantine", Color::new(189, 51, 164, 255));
        m.insert("byzantium", Color::new(112, 41, 99, 255));
        m.insert("cadet", Color::new(83, 104, 114, 255));
        m.insert("cadet_blue", Color::new(95, 158, 160, 255));
        m.insert("cadet_grey", Color::new(145, 163, 176, 255));
        m.insert("cadmium_green", Color::new(0, 107, 60, 255));
        m.insert("cadmium_orange", Color::new(237, 135, 45, 255));
        m.insert("cadmium_red", Color::new(227, 0, 34, 255));
        m.insert("cadmium_yellow", Color::new(255, 246, 0, 255));
        m.insert("caf_au_lait", Color::new(166, 123, 91, 255));
        m.insert("caf_noir", Color::new(75, 54, 33, 255));
        m.insert("cal_poly_green", Color::new(30, 77, 43, 255));
        m.insert("cambridge_blue", Color::new(163, 193, 173, 255));
        m.insert("camel", Color::new(193, 154, 107, 255));
        m.insert("cameo_pink", Color::new(239, 187, 204, 255));
        m.insert("camouflage_green", Color::new(120, 134, 107, 255));
        m.insert("canary_yellow", Color::new(255, 239, 0, 255));
        m.insert("candy_apple_red", Color::new(255, 8, 0, 255));
        m.insert("candy_pink", Color::new(228, 113, 122, 255));
        m.insert("capri", Color::new(0, 191, 255, 255));
        m.insert("caput_mortuum", Color::new(89, 39, 32, 255));
        m.insert("cardinal", Color::new(196, 30, 58, 255));
        m.insert("caribbean_green", Color::new(0, 204, 153, 255));
        m.insert("carmine", Color::new(150, 0, 24, 255));
        m.insert("carmine_m_p", Color::new(215, 0, 64, 255));
        m.insert("carmine_pink", Color::new(235, 76, 66, 255));
        m.insert("carmine_red", Color::new(255, 0, 56, 255));
        m.insert("carnation_pink", Color::new(255, 166, 201, 255));
        m.insert("carnelian", Color::new(179, 27, 27, 255));
        m.insert("carolina_blue", Color::new(153, 186, 221, 255));
        m.insert("carrot_orange", Color::new(237, 145, 33, 255));
        m.insert("catalina_blue", Color::new(6, 42, 120, 255));
        m.insert("ceil", Color::new(146, 161, 207, 255));
        m.insert("celadon", Color::new(172, 225, 175, 255));
        m.insert("celadon_blue", Color::new(0, 123, 167, 255));
        m.insert("celadon_green", Color::new(47, 132, 124, 255));
        m.insert("celeste_colour", Color::new(178, 255, 255, 255));
        m.insert("celestial_blue", Color::new(73, 151, 208, 255));
        m.insert("cerise", Color::new(222, 49, 99, 255));
        m.insert("cerise_pink", Color::new(236, 59, 131, 255));
        m.insert("cerulean", Color::new(0, 123, 167, 255));
        m.insert("cerulean_blue", Color::new(42, 82, 190, 255));
        m.insert("cerulean_frost", Color::new(109, 155, 195, 255));
        m.insert("cg_blue", Color::new(0, 122, 165, 255));
        m.insert("cg_red", Color::new(224, 60, 49, 255));
        m.insert("chamoisee", Color::new(160, 120, 90, 255));
        m.insert("champagne", Color::new(250, 214, 165, 255));
        m.insert("charcoal", Color::new(54, 69, 79, 255));
        m.insert("charm_pink", Color::new(230, 143, 172, 255));
        m.insert("chartreuse_traditional", Color::new(223, 255, 0, 255));
        m.insert("chartreuse_web", Color::new(127, 255, 0, 255));
        m.insert("cherry", Color::new(222, 49, 99, 255));
        m.insert("cherry_blossom_pink", Color::new(255, 183, 197, 255));
        m.insert("chestnut", Color::new(205, 92, 92, 255));
        m.insert("china_pink", Color::new(222, 111, 161, 255));
        m.insert("china_rose", Color::new(168, 81, 110, 255));
        m.insert("chinese_red", Color::new(170, 56, 30, 255));
        m.insert("chocolate_traditional", Color::new(123, 63, 0, 255));
        m.insert("chocolate_web", Color::new(210, 105, 30, 255));
        m.insert("chrome_yellow", Color::new(255, 167, 0, 255));
        m.insert("cinereous", Color::new(152, 129, 123, 255));
        m.insert("cinnabar", Color::new(227, 66, 52, 255));
        m.insert("cinnamon", Color::new(210, 105, 30, 255));
        m.insert("citrine", Color::new(228, 208, 10, 255));
        m.insert("classic_rose", Color::new(251, 204, 231, 255));
        m.insert("cobalt", Color::new(0, 71, 171, 255));
        m.insert("cocoa_brown", Color::new(210, 105, 30, 255));
        m.insert("coffee", Color::new(111, 78, 55, 255));
        m.insert("columbia_blue", Color::new(155, 221, 255, 255));
        m.insert("congo_pink", Color::new(248, 131, 121, 255));
        m.insert("cool_black", Color::new(0, 46, 99, 255));
        m.insert("cool_grey", Color::new(140, 146, 172, 255));
        m.insert("copper", Color::new(184, 115, 51, 255));
        m.insert("copper_crayola", Color::new(218, 138, 103, 255));
        m.insert("copper_penny", Color::new(173, 111, 105, 255));
        m.insert("copper_red", Color::new(203, 109, 81, 255));
        m.insert("copper_rose", Color::new(153, 102, 102, 255));
        m.insert("coquelicot", Color::new(255, 56, 0, 255));
        m.insert("coral", Color::new(255, 127, 80, 255));
        m.insert("coral_pink", Color::new(248, 131, 121, 255));
        m.insert("coral_red", Color::new(255, 64, 64, 255));
        m.insert("cordovan", Color::new(137, 63, 69, 255));
        m.insert("corn", Color::new(251, 236, 93, 255));
        m.insert("cornell_red", Color::new(179, 27, 27, 255));
        m.insert("cornflower_blue", Color::new(100, 149, 237, 255));
        m.insert("cornsilk", Color::new(255, 248, 220, 255));
        m.insert("cosmic_latte", Color::new(255, 248, 231, 255));
        m.insert("cotton_candy", Color::new(255, 188, 217, 255));
        m.insert("cream", Color::new(255, 253, 208, 255));
        m.insert("crimson", Color::new(220, 20, 60, 255));
        m.insert("crimson_glory", Color::new(190, 0, 50, 255));
        m.insert("cyan", Color::new(0, 255, 255, 255));
        m.insert("cyan_process", Color::new(0, 183, 235, 255));
        m.insert("daffodil", Color::new(255, 255, 49, 255));
        m.insert("dandelion", Color::new(240, 225, 48, 255));
        m.insert("dark_blue", Color::new(0, 0, 139, 255));
        m.insert("dark_brown", Color::new(101, 67, 33, 255));
        m.insert("dark_byzantium", Color::new(93, 57, 84, 255));
        m.insert("dark_candy_apple_red", Color::new(164, 0, 0, 255));
        m.insert("dark_cerulean", Color::new(8, 69, 126, 255));
        m.insert("dark_chestnut", Color::new(152, 105, 96, 255));
        m.insert("dark_coral", Color::new(205, 91, 69, 255));
        m.insert("dark_cyan", Color::new(0, 139, 139, 255));
        m.insert("dark_electric_blue", Color::new(83, 104, 120, 255));
        m.insert("dark_goldenrod", Color::new(184, 134, 11, 255));
        m.insert("dark_gray", Color::new(169, 169, 169, 255));
        m.insert("dark_green", Color::new(1, 50, 32, 255));
        m.insert("dark_imperial_blue", Color::new(0, 65, 106, 255));
        m.insert("dark_jungle_green", Color::new(26, 36, 33, 255));
        m.insert("dark_khaki", Color::new(189, 183, 107, 255));
        m.insert("dark_lava", Color::new(72, 60, 50, 255));
        m.insert("dark_lavender", Color::new(115, 79, 150, 255));
        m.insert("dark_magenta", Color::new(139, 0, 139, 255));
        m.insert("dark_midnight_blue", Color::new(0, 51, 102, 255));
        m.insert("dark_olive_green", Color::new(85, 107, 47, 255));
        m.insert("dark_orange", Color::new(255, 140, 0, 255));
        m.insert("dark_orchid", Color::new(153, 50, 204, 255));
        m.insert("dark_pastel_blue", Color::new(119, 158, 203, 255));
        m.insert("dark_pastel_green", Color::new(3, 192, 60, 255));
        m.insert("dark_pastel_purple", Color::new(150, 111, 214, 255));
        m.insert("dark_pastel_red", Color::new(194, 59, 34, 255));
        m.insert("dark_pink", Color::new(231, 84, 128, 255));
        m.insert("dark_powder_blue", Color::new(0, 51, 153, 255));
        m.insert("dark_raspberry", Color::new(135, 38, 87, 255));
        m.insert("dark_red", Color::new(139, 0, 0, 255));
        m.insert("dark_salmon", Color::new(233, 150, 122, 255));
        m.insert("dark_scarlet", Color::new(86, 3, 25, 255));
        m.insert("dark_sea_green", Color::new(143, 188, 143, 255));
        m.insert("dark_sienna", Color::new(60, 20, 20, 255));
        m.insert("dark_slate_blue", Color::new(72, 61, 139, 255));
        m.insert("dark_slate_gray", Color::new(47, 79, 79, 255));
        m.insert("dark_spring_green", Color::new(23, 114, 69, 255));
        m.insert("dark_tan", Color::new(145, 129, 81, 255));
        m.insert("dark_tangerine", Color::new(255, 168, 18, 255));
        m.insert("dark_taupe", Color::new(72, 60, 50, 255));
        m.insert("dark_terra_cotta", Color::new(204, 78, 92, 255));
        m.insert("dark_turquoise", Color::new(0, 206, 209, 255));
        m.insert("dark_violet", Color::new(148, 0, 211, 255));
        m.insert("dark_yellow", Color::new(155, 135, 12, 255));
        m.insert("dartmouth_green", Color::new(0, 112, 60, 255));
        m.insert("davy_s_grey", Color::new(85, 85, 85, 255));
        m.insert("debian_red", Color::new(215, 10, 83, 255));
        m.insert("deep_carmine", Color::new(169, 32, 62, 255));
        m.insert("deep_carmine_pink", Color::new(239, 48, 56, 255));
        m.insert("deep_carrot_orange", Color::new(233, 105, 44, 255));
        m.insert("deep_cerise", Color::new(218, 50, 135, 255));
        m.insert("deep_champagne", Color::new(250, 214, 165, 255));
        m.insert("deep_chestnut", Color::new(185, 78, 72, 255));
        m.insert("deep_coffee", Color::new(112, 66, 65, 255));
        m.insert("deep_fuchsia", Color::new(193, 84, 193, 255));
        m.insert("deep_jungle_green", Color::new(0, 75, 73, 255));
        m.insert("deep_lilac", Color::new(153, 85, 187, 255));
        m.insert("deep_magenta", Color::new(204, 0, 204, 255));
        m.insert("deep_peach", Color::new(255, 203, 164, 255));
        m.insert("deep_pink", Color::new(255, 20, 147, 255));
        m.insert("deep_ruby", Color::new(132, 63, 91, 255));
        m.insert("deep_saffron", Color::new(255, 153, 51, 255));
        m.insert("deep_sky_blue", Color::new(0, 191, 255, 255));
        m.insert("deep_tuscan_red", Color::new(102, 66, 77, 255));
        m.insert("denim", Color::new(21, 96, 189, 255));
        m.insert("desert", Color::new(193, 154, 107, 255));
        m.insert("desert_sand", Color::new(237, 201, 175, 255));
        m.insert("dim_gray", Color::new(105, 105, 105, 255));
        m.insert("dodger_blue", Color::new(30, 144, 255, 255));
        m.insert("dogwood_rose", Color::new(215, 24, 104, 255));
        m.insert("dollar_bill", Color::new(133, 187, 101, 255));
        m.insert("drab", Color::new(150, 113, 23, 255));
        m.insert("duke_blue", Color::new(0, 0, 156, 255));
        m.insert("earth_yellow", Color::new(225, 169, 95, 255));
        m.insert("ebony", Color::new(85, 93, 80, 255));
        m.insert("ecru", Color::new(194, 178, 128, 255));
        m.insert("eggplant", Color::new(97, 64, 81, 255));
        m.insert("eggshell", Color::new(240, 234, 214, 255));
        m.insert("egyptian_blue", Color::new(16, 52, 166, 255));
        m.insert("electric_blue", Color::new(125, 249, 255, 255));
        m.insert("electric_crimson", Color::new(255, 0, 63, 255));
        m.insert("electric_cyan", Color::new(0, 255, 255, 255));
        m.insert("electric_green", Color::new(0, 255, 0, 255));
        m.insert("electric_indigo", Color::new(111, 0, 255, 255));
        m.insert("electric_lavender", Color::new(244, 187, 255, 255));
        m.insert("electric_lime", Color::new(204, 255, 0, 255));
        m.insert("electric_purple", Color::new(191, 0, 255, 255));
        m.insert("electric_ultramarine", Color::new(63, 0, 255, 255));
        m.insert("electric_violet", Color::new(143, 0, 255, 255));
        m.insert("electric_yellow", Color::new(255, 255, 0, 255));
        m.insert("emerald", Color::new(80, 200, 120, 255));
        m.insert("english_lavender", Color::new(180, 131, 149, 255));
        m.insert("eton_blue", Color::new(150, 200, 162, 255));
        m.insert("fallow", Color::new(193, 154, 107, 255));
        m.insert("falu_red", Color::new(128, 24, 24, 255));
        m.insert("fandango", Color::new(181, 51, 137, 255));
        m.insert("fashion_fuchsia", Color::new(244, 0, 161, 255));
        m.insert("fawn", Color::new(229, 170, 112, 255));
        m.insert("feldgrau", Color::new(77, 93, 83, 255));
        m.insert("fern_green", Color::new(79, 121, 66, 255));
        m.insert("ferrari_red", Color::new(255, 40, 0, 255));
        m.insert("field_drab", Color::new(108, 84, 30, 255));
        m.insert("fire_engine_red", Color::new(206, 32, 41, 255));
        m.insert("firebrick", Color::new(178, 34, 34, 255));
        m.insert("flame", Color::new(226, 88, 34, 255));
        m.insert("flamingo_pink", Color::new(252, 142, 172, 255));
        m.insert("flavescent", Color::new(247, 233, 142, 255));
        m.insert("flax", Color::new(238, 220, 130, 255));
        m.insert("floral_white", Color::new(255, 250, 240, 255));
        m.insert("fluorescent_orange", Color::new(255, 191, 0, 255));
        m.insert("fluorescent_pink", Color::new(255, 20, 147, 255));
        m.insert("fluorescent_yellow", Color::new(204, 255, 0, 255));
        m.insert("folly", Color::new(255, 0, 79, 255));
        m.insert("forest_green_traditional", Color::new(1, 68, 33, 255));
        m.insert("forest_green_web", Color::new(34, 139, 34, 255));
        m.insert("french_beige", Color::new(166, 123, 91, 255));
        m.insert("french_blue", Color::new(0, 114, 187, 255));
        m.insert("french_lilac", Color::new(134, 96, 142, 255));
        m.insert("french_lime", Color::new(204, 255, 0, 255));
        m.insert("french_raspberry", Color::new(199, 44, 72, 255));
        m.insert("french_rose", Color::new(246, 74, 138, 255));
        m.insert("fuchsia", Color::new(255, 0, 255, 255));
        m.insert("fuchsia_crayola", Color::new(193, 84, 193, 255));
        m.insert("fuchsia_pink", Color::new(255, 119, 255, 255));
        m.insert("fuchsia_rose", Color::new(199, 67, 117, 255));
        m.insert("fulvous", Color::new(228, 132, 0, 255));
        m.insert("fuzzy_wuzzy", Color::new(204, 102, 102, 255));
        m.insert("gainsboro", Color::new(220, 220, 220, 255));
        m.insert("gamboge", Color::new(228, 155, 15, 255));
        m.insert("ghost_white", Color::new(248, 248, 255, 255));
        m.insert("ginger", Color::new(176, 101, 0, 255));
        m.insert("glaucous", Color::new(96, 130, 182, 255));
        m.insert("glitter", Color::new(230, 232, 250, 255));
        m.insert("gold_metallic", Color::new(212, 175, 55, 255));
        m.insert("gold_web_golden", Color::new(255, 215, 0, 255));
        m.insert("golden_brown", Color::new(153, 101, 21, 255));
        m.insert("golden_poppy", Color::new(252, 194, 0, 255));
        m.insert("golden_yellow", Color::new(255, 223, 0, 255));
        m.insert("goldenrod", Color::new(218, 165, 32, 255));
        m.insert("granny_smith_apple", Color::new(168, 228, 160, 255));
        m.insert("gray", Color::new(128, 128, 128, 255));
        m.insert("gray_asparagus", Color::new(70, 89, 69, 255));
        m.insert("gray_html_css_gray", Color::new(128, 128, 128, 255));
        m.insert("gray_x11_gray", Color::new(190, 190, 190, 255));
        m.insert("green_color_wheel_x11_green", Color::new(0, 255, 0, 255));
        m.insert("green_crayola", Color::new(28, 172, 120, 255));
        m.insert("green_html_css_green", Color::new(0, 128, 0, 255));
        m.insert("green_munsell", Color::new(0, 168, 119, 255));
        m.insert("green_ncs", Color::new(0, 159, 107, 255));
        m.insert("green_pigment", Color::new(0, 165, 80, 255));
        m.insert("green_ryb", Color::new(102, 176, 50, 255));
        m.insert("green_yellow", Color::new(173, 255, 47, 255));
        m.insert("grullo", Color::new(169, 154, 134, 255));
        m.insert("guppie_green", Color::new(0, 255, 127, 255));
        m.insert("halay_be", Color::new(102, 56, 84, 255));
        m.insert("han_blue", Color::new(68, 108, 207, 255));
        m.insert("han_purple", Color::new(82, 24, 250, 255));
        m.insert("hansa_yellow", Color::new(233, 214, 107, 255));
        m.insert("harlequin", Color::new(63, 255, 0, 255));
        m.insert("harvard_crimson", Color::new(201, 0, 22, 255));
        m.insert("harvest_gold", Color::new(218, 145, 0, 255));
        m.insert("heart_gold", Color::new(128, 128, 0, 255));
        m.insert("heliotrope", Color::new(223, 115, 255, 255));
        m.insert("hollywood_cerise", Color::new(244, 0, 161, 255));
        m.insert("honeydew", Color::new(240, 255, 240, 255));
        m.insert("honolulu_blue", Color::new(0, 127, 191, 255));
        m.insert("hooker_s_green", Color::new(73, 121, 107, 255));
        m.insert("hot_magenta", Color::new(255, 29, 206, 255));
        m.insert("hot_pink", Color::new(255, 105, 180, 255));
        m.insert("hunter_green", Color::new(53, 94, 59, 255));
        m.insert("iceberg", Color::new(113, 166, 210, 255));
        m.insert("icterine", Color::new(252, 247, 94, 255));
        m.insert("imperial_blue", Color::new(0, 35, 149, 255));
        m.insert("inchworm", Color::new(178, 236, 93, 255));
        m.insert("india_green", Color::new(19, 136, 8, 255));
        m.insert("indian_red", Color::new(205, 92, 92, 255));
        m.insert("indian_yellow", Color::new(227, 168, 87, 255));
        m.insert("indigo", Color::new(111, 0, 255, 255));
        m.insert("indigo_dye", Color::new(0, 65, 106, 255));
        m.insert("indigo_web", Color::new(75, 0, 130, 255));
        m.insert("international_klein_blue", Color::new(0, 47, 167, 255));
        m.insert("international_orange_aerospace", Color::new(255, 79, 0, 255));
        m.insert(
            "international_orange_engineering",
            Color::new(186, 22, 12, 255),
        );
        m.insert(
            "international_orange_golden_gate_bridge",
            Color::new(192, 54, 44, 255),
        );
        m.insert("iris", Color::new(90, 79, 207, 255));
        m.insert("isabelline", Color::new(244, 240, 236, 255));
        m.insert("islamic_green", Color::new(0, 144, 0, 255));
        m.insert("ivory", Color::new(255, 255, 240, 255));
        m.insert("jade", Color::new(0, 168, 107, 255));
        m.insert("jasmine", Color::new(248, 222, 126, 255));
        m.insert("jasper", Color::new(215, 59, 62, 255));
        m.insert("jazzberry_jam", Color::new(165, 11, 94, 255));
        m.insert("jet", Color::new(52, 52, 52, 255));
        m.insert("jonquil", Color::new(250, 218, 94, 255));
        m.insert("june_bud", Color::new(189, 218, 87, 255));
        m.insert("jungle_green", Color::new(41, 171, 135, 255));
        m.insert("kelly_green", Color::new(76, 187, 23, 255));
        m.insert("kenyan_copper", Color::new(124, 28, 5, 255));
        m.insert("khaki_html_css_khaki", Color::new(195, 176, 145, 255));
        m.insert("khaki_x11_light_khaki", Color::new(240, 230, 140, 255));
        m.insert("ku_crimson", Color::new(232, 0, 13, 255));
        m.insert("la_salle_green", Color::new(8, 120, 48, 255));
        m.insert("languid_lavender", Color::new(214, 202, 221, 255));
        m.insert("lapis_lazuli", Color::new(38, 97, 156, 255));
        m.insert("laser_lemon", Color::new(254, 254, 34, 255));
        m.insert("laurel_green", Color::new(169, 186, 157, 255));
        m.insert("lava", Color::new(207, 16, 32, 255));
        m.insert("lavender_blue", Color::new(204, 204, 255, 255));
        m.insert("lavender_blush", Color::new(255, 240, 245, 255));
        m.insert("lavender_floral", Color::new(181, 126, 220, 255));
        m.insert("lavender_gray", Color::new(196, 195, 208, 255));
        m.insert("lavender_indigo", Color::new(148, 87, 235, 255));
        m.insert("lavender_magenta", Color::new(238, 130, 238, 255));
        m.insert("lavender_mist", Color::new(230, 230, 250, 255));
        m.insert("lavender_pink", Color::new(251, 174, 210, 255));
        m.insert("lavender_purple", Color::new(150, 123, 182, 255));
        m.insert("lavender_rose", Color::new(251, 160, 227, 255));
        m.insert("lavender_web", Color::new(230, 230, 250, 255));
        m.insert("lawn_green", Color::new(124, 252, 0, 255));
        m.insert("lemon", Color::new(255, 247, 0, 255));
        m.insert("lemon_chiffon", Color::new(255, 250, 205, 255));
        m.insert("lemon_lime", Color::new(227, 255, 0, 255));
        m.insert("licorice", Color::new(26, 17, 16, 255));
        m.insert("light_apricot", Color::new(253, 213, 177, 255));
        m.insert("light_blue", Color::new(173, 216, 230, 255));
        m.insert("light_brown", Color::new(181, 101, 29, 255));
        m.insert("light_carmine_pink", Color::new(230, 103, 113, 255));
        m.insert("light_coral", Color::new(240, 128, 128, 255));
        m.insert("light_cornflower_blue", Color::new(147, 204, 234, 255));
        m.insert("light_crimson", Color::new(245, 105, 145, 255));
        m.insert("light_cyan", Color::new(224, 255, 255, 255));
        m.insert("light_fuchsia_pink", Color::new(249, 132, 239, 255));
        m.insert("light_goldenrod_yellow", Color::new(250, 250, 210, 255));
        m.insert("light_gray", Color::new(211, 211, 211, 255));
        m.insert("light_green", Color::new(144, 238, 144, 255));
        m.insert("light_khaki", Color::new(240, 230, 140, 255));
        m.insert("light_pastel_purple", Color::new(177, 156, 217, 255));
        m.insert("light_pink", Color::new(255, 182, 193, 255));
        m.insert("light_red_ochre", Color::new(233, 116, 81, 255));
        m.insert("light_salmon", Color::new(255, 160, 122, 255));
        m.insert("light_salmon_pink", Color::new(255, 153, 153, 255));
        m.insert("light_sea_green", Color::new(32, 178, 170, 255));
        m.insert("light_sky_blue", Color::new(135, 206, 250, 255));
        m.insert("light_slate_gray", Color::new(119, 136, 153, 255));
        m.insert("light_taupe", Color::new(179, 139, 109, 255));
        m.insert("light_thulian_pink", Color::new(230, 143, 172, 255));
        m.insert("light_yellow", Color::new(255, 255, 224, 255));
        m.insert("lilac", Color::new(200, 162, 200, 255));
        m.insert("lime_color_wheel", Color::new(191, 255, 0, 255));
        m.insert("lime_green", Color::new(50, 205, 50, 255));
        m.insert("lime_web_x11_green", Color::new(0, 255, 0, 255));
        m.insert("limerick", Color::new(157, 194, 9, 255));
        m.insert("lincoln_green", Color::new(25, 89, 5, 255));
        m.insert("linen", Color::new(250, 240, 230, 255));
        m.insert("lion", Color::new(193, 154, 107, 255));
        m.insert("little_boy_blue", Color::new(108, 160, 220, 255));
        m.insert("liver", Color::new(83, 75, 79, 255));
        m.insert("lust", Color::new(230, 32, 32, 255));
        m.insert("magenta", Color::new(255, 0, 255, 255));
        m.insert("magenta_dye", Color::new(202, 31, 123, 255));
        m.insert("magenta_process", Color::new(255, 0, 144, 255));
        m.insert("magic_mint", Color::new(170, 240, 209, 255));
        m.insert("magnolia", Color::new(248, 244, 255, 255));
        m.insert("mahogany", Color::new(192, 64, 0, 255));
        m.insert("maize", Color::new(251, 236, 93, 255));
        m.insert("majorelle_blue", Color::new(96, 80, 220, 255));
        m.insert("malachite", Color::new(11, 218, 81, 255));
        m.insert("manatee", Color::new(151, 154, 170, 255));
        m.insert("mango_tango", Color::new(255, 130, 67, 255));
        m.insert("mantis", Color::new(116, 195, 101, 255));
        m.insert("mardi_gras", Color::new(136, 0, 133, 255));
        m.insert("maroon_crayola", Color::new(195, 33, 72, 255));
        m.insert("maroon_html_css", Color::new(128, 0, 0, 255));
        m.insert("maroon_x11", Color::new(176, 48, 96, 255));
        m.insert("mauve", Color::new(224, 176, 255, 255));
        m.insert("mauve_taupe", Color::new(145, 95, 109, 255));
        m.insert("mauvelous", Color::new(239, 152, 170, 255));
        m.insert("maya_blue", Color::new(115, 194, 251, 255));
        m.insert("meat_brown", Color::new(229, 183, 59, 255));
        m.insert("medium_aquamarine", Color::new(102, 221, 170, 255));
        m.insert("medium_blue", Color::new(0, 0, 205, 255));
        m.insert("medium_candy_apple_red", Color::new(226, 6, 44, 255));
        m.insert("medium_carmine", Color::new(175, 64, 53, 255));
        m.insert("medium_champagne", Color::new(243, 229, 171, 255));
        m.insert("medium_electric_blue", Color::new(3, 80, 150, 255));
        m.insert("medium_jungle_green", Color::new(28, 53, 45, 255));
        m.insert("medium_lavender_magenta", Color::new(221, 160, 221, 255));
        m.insert("medium_orchid", Color::new(186, 85, 211, 255));
        m.insert("medium_persian_blue", Color::new(0, 103, 165, 255));
        m.insert("medium_purple", Color::new(147, 112, 219, 255));
        m.insert("medium_red_violet", Color::new(187, 51, 133, 255));
        m.insert("medium_ruby", Color::new(170, 64, 105, 255));
        m.insert("medium_sea_green", Color::new(60, 179, 113, 255));
        m.insert("medium_slate_blue", Color::new(123, 104, 238, 255));
        m.insert("medium_spring_bud", Color::new(201, 220, 135, 255));
        m.insert("medium_spring_green", Color::new(0, 250, 154, 255));
        m.insert("medium_taupe", Color::new(103, 76, 71, 255));
        m.insert("medium_turquoise", Color::new(72, 209, 204, 255));
        m.insert("medium_tuscan_red", Color::new(121, 68, 59, 255));
        m.insert("medium_vermilion", Color::new(217, 96, 59, 255));
        m.insert("medium_violet_red", Color::new(199, 21, 133, 255));
        m.insert("mellow_apricot", Color::new(248, 184, 120, 255));
        m.insert("mellow_yellow", Color::new(248, 222, 126, 255));
        m.insert("melon", Color::new(253, 188, 180, 255));
        m.insert("midnight_blue", Color::new(25, 25, 112, 255));
        m.insert("midnight_green_eagle_green", Color::new(0, 73, 83, 255));
        m.insert("mikado_yellow", Color::new(255, 196, 12, 255));
        m.insert("mint", Color::new(62, 180, 137, 255));
        m.insert("mint_cream", Color::new(245, 255, 250, 255));
        m.insert("mint_green", Color::new(152, 255, 152, 255));
        m.insert("misty_rose", Color::new(255, 228, 225, 255));
        m.insert("moccasin", Color::new(250, 235, 215, 255));
        m.insert("mode_beige", Color::new(150, 113, 23, 255));
        m.insert("moonstone_blue", Color::new(115, 169, 194, 255));
        m.insert("mordant_red_19", Color::new(174, 12, 0, 255));
        m.insert("moss_green", Color::new(173, 223, 173, 255));
        m.insert("mountain_meadow", Color::new(48, 186, 143, 255));
        m.insert("mountbatten_pink", Color::new(153, 122, 141, 255));
        m.insert("msu_green", Color::new(24, 69, 59, 255));
        m.insert("mulberry", Color::new(197, 75, 140, 255));
        m.insert("mustard", Color::new(255, 219, 88, 255));
        m.insert("myrtle", Color::new(33, 66, 30, 255));
        m.insert("nadeshiko_pink", Color::new(246, 173, 198, 255));
        m.insert("napier_green", Color::new(42, 128, 0, 255));
        m.insert("naples_yellow", Color::new(250, 218, 94, 255));
        m.insert("navajo_white", Color::new(255, 222, 173, 255));
        m.insert("navy_blue", Color::new(0, 0, 128, 255));
        m.insert("neon_carrot", Color::new(255, 163, 67, 255));
        m.insert("neon_fuchsia", Color::new(254, 65, 100, 255));
        m.insert("neon_green", Color::new(57, 255, 20, 255));
        m.insert("new_york_pink", Color::new(215, 131, 127, 255));
        m.insert("non_photo_blue", Color::new(164, 221, 237, 255));
        m.insert("north_texas_green", Color::new(5, 144, 51, 255));
        m.insert("ocean_boat_blue", Color::new(0, 119, 190, 255));
        m.insert("ochre", Color::new(204, 119, 34, 255));
        m.insert("office_green", Color::new(0, 128, 0, 255));
        m.insert("old_gold", Color::new(207, 181, 59, 255));
        m.insert("old_lace", Color::new(253, 245, 230, 255));
        m.insert("old_lavender", Color::new(121, 104, 120, 255));
        m.insert("old_mauve", Color::new(103, 49, 71, 255));
        m.insert("old_rose", Color::new(192, 128, 129, 255));
        m.insert("olive", Color::new(128, 128, 0, 255));
        m.insert("olive_drab_7", Color::new(60, 52, 31, 255));
        m.insert("olive_drab_web_olive_drab_3", Color::new(107, 142, 35, 255));
        m.insert("olivine", Color::new(154, 185, 115, 255));
        m.insert("onyx", Color::new(53, 56, 57, 255));
        m.insert("opera_mauve", Color::new(183, 132, 167, 255));
        m.insert("orange_color_wheel", Color::new(255, 127, 0, 255));
        m.insert("orange_peel", Color::new(255, 159, 0, 255));
        m.insert("orange_red", Color::new(255, 69, 0, 255));
        m.insert("orange_ryb", Color::new(251, 153, 2, 255));
        m.insert("orange_web_color", Color::new(255, 165, 0, 255));
        m.insert("orchid", Color::new(218, 112, 214, 255));
        m.insert("otter_brown", Color::new(101, 67, 33, 255));
        m.insert("ou_crimson_red", Color::new(153, 0, 0, 255));
        m.insert("outer_space", Color::new(65, 74, 76, 255));
        m.insert("outrageous_orange", Color::new(255, 110, 74, 255));
        m.insert("oxford_blue", Color::new(0, 33, 71, 255));
        m.insert("pakistan_green", Color::new(0, 102, 0, 255));
        m.insert("palatinate_blue", Color::new(39, 59, 226, 255));
        m.insert("palatinate_purple", Color::new(104, 40, 96, 255));
        m.insert("pale_aqua", Color::new(188, 212, 230, 255));
        m.insert("pale_blue", Color::new(175, 238, 238, 255));
        m.insert("pale_brown", Color::new(152, 118, 84, 255));
        m.insert("pale_carmine", Color::new(175, 64, 53, 255));
        m.insert("pale_cerulean", Color::new(155, 196, 226, 255));
        m.insert("pale_chestnut", Color::new(221, 173, 175, 255));
        m.insert("pale_copper", Color::new(218, 138, 103, 255));
        m.insert("pale_cornflower_blue", Color::new(171, 205, 239, 255));
        m.insert("pale_gold", Color::new(230, 190, 138, 255));
        m.insert("pale_goldenrod", Color::new(238, 232, 170, 255));
        m.insert("pale_green", Color::new(152, 251, 152, 255));
        m.insert("pale_lavender", Color::new(220, 208, 255, 255));
        m.insert("pale_magenta", Color::new(249, 132, 229, 255));
        m.insert("pale_pink", Color::new(250, 218, 221, 255));
        m.insert("pale_plum", Color::new(221, 160, 221, 255));
        m.insert("pale_red_violet", Color::new(219, 112, 147, 255));
        m.insert("pale_robin_egg_blue", Color::new(150, 222, 209, 255));
        m.insert("pale_silver", Color::new(201, 192, 187, 255));
        m.insert("pale_spring_bud", Color::new(236, 235, 189, 255));
        m.insert("pale_taupe", Color::new(188, 152, 126, 255));
        m.insert("pale_violet_red", Color::new(219, 112, 147, 255));
        m.insert("pansy_purple", Color::new(120, 24, 74, 255));
        m.insert("papaya_whip", Color::new(255, 239, 213, 255));
        m.insert("paris_green", Color::new(80, 200, 120, 255));
        m.insert("pastel_blue", Color::new(174, 198, 207, 255));
        m.insert("pastel_brown", Color::new(131, 105, 83, 255));
        m.insert("pastel_gray", Color::new(207, 207, 196, 255));
        m.insert("pastel_green", Color::new(119, 221, 119, 255));
        m.insert("pastel_magenta", Color::new(244, 154, 194, 255));
        m.insert("pastel_orange", Color::new(255, 179, 71, 255));
        m.insert("pastel_pink", Color::new(222, 165, 164, 255));
        m.insert("pastel_purple", Color::new(179, 158, 181, 255));
        m.insert("pastel_red", Color::new(255, 105, 97, 255));
        m.insert("pastel_violet", Color::new(203, 153, 201, 255));
        m.insert("pastel_yellow", Color::new(253, 253, 150, 255));
        m.insert("patriarch", Color::new(128, 0, 128, 255));
        m.insert("payne_s_grey", Color::new(83, 104, 120, 255));
        m.insert("peach", Color::new(255, 229, 180, 255));
        m.insert("peach_crayola", Color::new(255, 203, 164, 255));
        m.insert("peach_orange", Color::new(255, 204, 153, 255));
        m.insert("peach_puff", Color::new(255, 218, 185, 255));
        m.insert("peach_yellow", Color::new(250, 223, 173, 255));
        m.insert("pear", Color::new(209, 226, 49, 255));
        m.insert("pearl", Color::new(234, 224, 200, 255));
        m.insert("pearl_aqua", Color::new(136, 216, 192, 255));
        m.insert("pearly_purple", Color::new(183, 104, 162, 255));
        m.insert("peridot", Color::new(230, 226, 0, 255));
        m.insert("periwinkle", Color::new(204, 204, 255, 255));
        m.insert("persian_blue", Color::new(28, 57, 187, 255));
        m.insert("persian_green", Color::new(0, 166, 147, 255));
        m.insert("persian_indigo", Color::new(50, 18, 122, 255));
        m.insert("persian_orange", Color::new(217, 144, 88, 255));
        m.insert("persian_pink", Color::new(247, 127, 190, 255));
        m.insert("persian_plum", Color::new(112, 28, 28, 255));
        m.insert("persian_red", Color::new(204, 51, 51, 255));
        m.insert("persian_rose", Color::new(254, 40, 162, 255));
        m.insert("persimmon", Color::new(236, 88, 0, 255));
        m.insert("peru", Color::new(205, 133, 63, 255));
        m.insert("phlox", Color::new(223, 0, 255, 255));
        m.insert("phthalo_blue", Color::new(0, 15, 137, 255));
        m.insert("phthalo_green", Color::new(18, 53, 36, 255));
        m.insert("piggy_pink", Color::new(253, 221, 230, 255));
        m.insert("pine_green", Color::new(1, 121, 111, 255));
        m.insert("pink", Color::new(255, 192, 203, 255));
        m.insert("pink_lace", Color::new(255, 221, 244, 255));
        m.insert("pink_orange", Color::new(255, 153, 102, 255));
        m.insert("pink_pearl", Color::new(231, 172, 207, 255));
        m.insert("pink_sherbet", Color::new(247, 143, 167, 255));
        m.insert("pistachio", Color::new(147, 197, 114, 255));
        m.insert("platinum", Color::new(229, 228, 226, 255));
        m.insert("plum_traditional", Color::new(142, 69, 133, 255));
        m.insert("plum_web", Color::new(221, 160, 221, 255));
        m.insert("portland_orange", Color::new(255, 90, 54, 255));
        m.insert("powder_blue_web", Color::new(176, 224, 230, 255));
        m.insert("princeton_orange", Color::new(255, 143, 0, 255));
        m.insert("prune", Color::new(112, 28, 28, 255));
        m.insert("prussian_blue", Color::new(0, 49, 83, 255));
        m.insert("psychedelic_purple", Color::new(223, 0, 255, 255));
        m.insert("puce", Color::new(204, 136, 153, 255));
        m.insert("pumpkin", Color::new(255, 117, 24, 255));
        m.insert("purple_heart", Color::new(105, 53, 156, 255));
        m.insert("purple_html_css", Color::new(128, 0, 128, 255));
        m.insert("purple_mountain_majesty", Color::new(150, 120, 182, 255));
        m.insert("purple_munsell", Color::new(159, 0, 197, 255));
        m.insert("purple_pizzazz", Color::new(254, 78, 218, 255));
        m.insert("purple_taupe", Color::new(80, 64, 77, 255));
        m.insert("purple_x11", Color::new(160, 32, 240, 255));
        m.insert("quartz", Color::new(81, 72, 79, 255));
        m.insert("rackley", Color::new(93, 138, 168, 255));
        m.insert("radical_red", Color::new(255, 53, 94, 255));
        m.insert("rajah", Color::new(251, 171, 96, 255));
        m.insert("raspberry", Color::new(227, 11, 93, 255));
        m.insert("raspberry_glace", Color::new(145, 95, 109, 255));
        m.insert("raspberry_pink", Color::new(226, 80, 152, 255));
        m.insert("raspberry_rose", Color::new(179, 68, 108, 255));
        m.insert("raw_umber", Color::new(130, 102, 68, 255));
        m.insert("razzle_dazzle_rose", Color::new(255, 51, 204, 255));
        m.insert("razzmatazz", Color::new(227, 37, 107, 255));
        m.insert("red", Color::new(255, 0, 0, 255));
        m.insert("red_brown", Color::new(165, 42, 42, 255));
        m.insert("red_devil", Color::new(134, 1, 17, 255));
        m.insert("red_munsell", Color::new(242, 0, 60, 255));
        m.insert("red_ncs", Color::new(196, 2, 51, 255));
        m.insert("red_orange", Color::new(255, 83, 73, 255));
        m.insert("red_pigment", Color::new(237, 28, 36, 255));
        m.insert("red_ryb", Color::new(254, 39, 18, 255));
        m.insert("red_violet", Color::new(199, 21, 133, 255));
        m.insert("redwood", Color::new(171, 78, 82, 255));
        m.insert("regalia", Color::new(82, 45, 128, 255));
        m.insert("resolution_blue", Color::new(0, 35, 135, 255));
        m.insert("rich_black", Color::new(0, 64, 64, 255));
        m.insert("rich_brilliant_lavender", Color::new(241, 167, 254, 255));
        m.insert("rich_carmine", Color::new(215, 0, 64, 255));
        m.insert("rich_electric_blue", Color::new(8, 146, 208, 255));
        m.insert("rich_lavender", Color::new(167, 107, 207, 255));
        m.insert("rich_lilac", Color::new(182, 102, 210, 255));
        m.insert("rich_maroon", Color::new(176, 48, 96, 255));
        m.insert("rifle_green", Color::new(65, 72, 51, 255));
        m.insert("robin_egg_blue", Color::new(0, 204, 204, 255));
        m.insert("rose", Color::new(255, 0, 127, 255));
        m.insert("rose_bonbon", Color::new(249, 66, 158, 255));
        m.insert("rose_ebony", Color::new(103, 72, 70, 255));
        m.insert("rose_gold", Color::new(183, 110, 121, 255));
        m.insert("rose_madder", Color::new(227, 38, 54, 255));
        m.insert("rose_pink", Color::new(255, 102, 204, 255));
        m.insert("rose_quartz", Color::new(170, 152, 169, 255));
        m.insert("rose_taupe", Color::new(144, 93, 93, 255));
        m.insert("rose_vale", Color::new(171, 78, 82, 255));
        m.insert("rosewood", Color::new(101, 0, 11, 255));
        m.insert("rosso_corsa", Color::new(212, 0, 0, 255));
        m.insert("rosy_brown", Color::new(188, 143, 143, 255));
        m.insert("royal_azure", Color::new(0, 56, 168, 255));
        m.insert("royal_blue_traditional", Color::new(0, 35, 102, 255));
        m.insert("royal_blue_web", Color::new(65, 105, 225, 255));
        m.insert("royal_fuchsia", Color::new(202, 44, 146, 255));
        m.insert("royal_purple", Color::new(120, 81, 169, 255));
        m.insert("royal_yellow", Color::new(250, 218, 94, 255));
        m.insert("rubine_red", Color::new(209, 0, 86, 255));
        m.insert("ruby", Color::new(224, 17, 95, 255));
        m.insert("ruby_red", Color::new(155, 17, 30, 255));
        m.insert("ruddy", Color::new(255, 0, 40, 255));
        m.insert("ruddy_brown", Color::new(187, 101, 40, 255));
        m.insert("ruddy_pink", Color::new(225, 142, 150, 255));
        m.insert("rufous", Color::new(168, 28, 7, 255));
        m.insert("russet", Color::new(128, 70, 27, 255));
        m.insert("rust", Color::new(183, 65, 14, 255));
        m.insert("rusty_red", Color::new(218, 44, 67, 255));
        m.insert("sacramento_state_green", Color::new(0, 86, 63, 255));
        m.insert("saddle_brown", Color::new(139, 69, 19, 255));
        m.insert("safety_orange_blaze_orange", Color::new(255, 103, 0, 255));
        m.insert("saffron", Color::new(244, 196, 48, 255));
        m.insert("salmon", Color::new(255, 140, 105, 255));
        m.insert("salmon_pink", Color::new(255, 145, 164, 255));
        m.insert("sand", Color::new(194, 178, 128, 255));
        m.insert("sand_dune", Color::new(150, 113, 23, 255));
        m.insert("sandstorm", Color::new(236, 213, 64, 255));
        m.insert("sandy_brown", Color::new(244, 164, 96, 255));
        m.insert("sandy_taupe", Color::new(150, 113, 23, 255));
        m.insert("sangria", Color::new(146, 0, 10, 255));
        m.insert("sap_green", Color::new(80, 125, 42, 255));
        m.insert("sapphire", Color::new(15, 82, 186, 255));
        m.insert("sapphire_blue", Color::new(0, 103, 165, 255));
        m.insert("satin_sheen_gold", Color::new(203, 161, 53, 255));
        m.insert("scarlet", Color::new(255, 36, 0, 255));
        m.insert("scarlet_crayola", Color::new(253, 14, 53, 255));
        m.insert("school_bus_yellow", Color::new(255, 216, 0, 255));
        m.insert("screamin_green", Color::new(118, 255, 122, 255));
        m.insert("sea_blue", Color::new(0, 105, 148, 255));
        m.insert("sea_green", Color::new(46, 139, 87, 255));
        m.insert("seal_brown", Color::new(50, 20, 20, 255));
        m.insert("seashell", Color::new(255, 245, 238, 255));
        m.insert("selective_yellow", Color::new(255, 186, 0, 255));
        m.insert("sepia", Color::new(112, 66, 20, 255));
        m.insert("shadow", Color::new(138, 121, 93, 255));
        m.insert("shamrock_green", Color::new(0, 158, 96, 255));
        m.insert("shocking_pink", Color::new(252, 15, 192, 255));
        m.insert("shocking_pink_crayola", Color::new(255, 111, 255, 255));
        m.insert("sienna", Color::new(136, 45, 23, 255));
        m.insert("silver", Color::new(192, 192, 192, 255));
        m.insert("sinopia", Color::new(203, 65, 11, 255));
        m.insert("skobeloff", Color::new(0, 116, 116, 255));
        m.insert("sky_blue", Color::new(135, 206, 235, 255));
        m.insert("sky_magenta", Color::new(207, 113, 175, 255));
        m.insert("slate_blue", Color::new(106, 90, 205, 255));
        m.insert("slate_gray", Color::new(112, 128, 144, 255));
        m.insert("smalt_dark_powder_blue", Color::new(0, 51, 153, 255));
        m.insert("smokey_topaz", Color::new(147, 61, 65, 255));
        m.insert("smoky_black", Color::new(16, 12, 8, 255));
        m.insert("snow", Color::new(255, 250, 250, 255));
        m.insert("spiro_disco_ball", Color::new(15, 192, 252, 255));
        m.insert("spring_bud", Color::new(167, 252, 0, 255));
        m.insert("spring_green", Color::new(0, 255, 127, 255));
        m.insert("st_patrick_s_blue", Color::new(35, 41, 122, 255));
        m.insert("steel_blue", Color::new(70, 130, 180, 255));
        m.insert("stil_de_grain_yellow", Color::new(250, 218, 94, 255));
        m.insert("stizza", Color::new(153, 0, 0, 255));
        m.insert("stormcloud", Color::new(79, 102, 106, 255));
        m.insert("straw", Color::new(228, 217, 111, 255));
        m.insert("sunglow", Color::new(255, 204, 51, 255));
        m.insert("sunset", Color::new(250, 214, 165, 255));
        m.insert("tan", Color::new(210, 180, 140, 255));
        m.insert("tangelo", Color::new(249, 77, 0, 255));
        m.insert("tangerine", Color::new(242, 133, 0, 255));
        m.insert("tangerine_yellow", Color::new(255, 204, 0, 255));
        m.insert("tango_pink", Color::new(228, 113, 122, 255));
        m.insert("taupe", Color::new(72, 60, 50, 255));
        m.insert("taupe_gray", Color::new(139, 133, 137, 255));
        m.insert("tea_green", Color::new(208, 240, 192, 255));
        m.insert("tea_rose_orange", Color::new(248, 131, 121, 255));
        m.insert("tea_rose_rose", Color::new(244, 194, 194, 255));
        m.insert("teal", Color::new(0, 128, 128, 255));
        m.insert("teal_blue", Color::new(54, 117, 136, 255));
        m.insert("teal_green", Color::new(0, 130, 127, 255));
        m.insert("telemagenta", Color::new(207, 52, 118, 255));
        m.insert("tenn_tawny", Color::new(205, 87, 0, 255));
        m.insert("terra_cotta", Color::new(226, 114, 91, 255));
        m.insert("thistle", Color::new(216, 191, 216, 255));
        m.insert("thulian_pink", Color::new(222, 111, 161, 255));
        m.insert("tickle_me_pink", Color::new(252, 137, 172, 255));
        m.insert("tiffany_blue", Color::new(10, 186, 181, 255));
        m.insert("tiger_s_eye", Color::new(224, 141, 60, 255));
        m.insert("timberwolf", Color::new(219, 215, 210, 255));
        m.insert("titanium_yellow", Color::new(238, 230, 0, 255));
        m.insert("tomato", Color::new(255, 99, 71, 255));
        m.insert("toolbox", Color::new(116, 108, 192, 255));
        m.insert("topaz", Color::new(255, 200, 124, 255));
        m.insert("tractor_red", Color::new(253, 14, 53, 255));
        m.insert("trolley_grey", Color::new(128, 128, 128, 255));
        m.insert("tropical_rain_forest", Color::new(0, 117, 94, 255));
        m.insert("true_blue", Color::new(0, 115, 207, 255));
        m.insert("tufts_blue", Color::new(65, 125, 193, 255));
        m.insert("tumbleweed", Color::new(222, 170, 136, 255));
        m.insert("turkish_rose", Color::new(181, 114, 129, 255));
        m.insert("turquoise", Color::new(48, 213, 200, 255));
        m.insert("turquoise_blue", Color::new(0, 255, 239, 255));
        m.insert("turquoise_green", Color::new(160, 214, 180, 255));
        m.insert("tuscan_red", Color::new(124, 72, 72, 255));
        m.insert("twilight_lavender", Color::new(138, 73, 107, 255));
        m.insert("tyrian_purple", Color::new(102, 2, 60, 255));
        m.insert("ua_blue", Color::new(0, 51, 170, 255));
        m.insert("ua_red", Color::new(217, 0, 76, 255));
        m.insert("ube", Color::new(136, 120, 195, 255));
        m.insert("ucla_blue", Color::new(83, 104, 149, 255));
        m.insert("ucla_gold", Color::new(255, 179, 0, 255));
        m.insert("ufo_green", Color::new(60, 208, 112, 255));
        m.insert("ultra_pink", Color::new(255, 111, 255, 255));
        m.insert("ultramarine", Color::new(18, 10, 143, 255));
        m.insert("ultramarine_blue", Color::new(65, 102, 245, 255));
        m.insert("umber", Color::new(99, 81, 71, 255));
        m.insert("unbleached_silk", Color::new(255, 221, 202, 255));
        m.insert("united_nations_blue", Color::new(91, 146, 229, 255));
        m.insert(
            "university_of_california_gold",
            Color::new(183, 135, 39, 255),
        );
        m.insert("unmellow_yellow", Color::new(255, 255, 102, 255));
        m.insert("up_forest_green", Color::new(1, 68, 33, 255));
        m.insert("up_maroon", Color::new(123, 17, 19, 255));
        m.insert("upsdell_red", Color::new(174, 32, 41, 255));
        m.insert("urobilin", Color::new(225, 173, 33, 255));
        m.insert("usafa_blue", Color::new(0, 79, 152, 255));
        m.insert("usc_cardinal", Color::new(153, 0, 0, 255));
        m.insert("usc_gold", Color::new(255, 204, 0, 255));
        m.insert("utah_crimson", Color::new(211, 0, 63, 255));
        m.insert("vanilla", Color::new(243, 229, 171, 255));
        m.insert("vegas_gold", Color::new(197, 179, 88, 255));
        m.insert("venetian_red", Color::new(200, 8, 21, 255));
        m.insert("verdigris", Color::new(67, 179, 174, 255));
        m.insert("vermilion_cinnabar", Color::new(227, 66, 52, 255));
        m.insert("vermilion_plochere", Color::new(217, 96, 59, 255));
        m.insert("veronica", Color::new(160, 32, 240, 255));
        m.insert("violet", Color::new(143, 0, 255, 255));
        m.insert("violet_blue", Color::new(50, 74, 178, 255));
        m.insert("violet_color_wheel", Color::new(127, 0, 255, 255));
        m.insert("violet_ryb", Color::new(134, 1, 175, 255));
        m.insert("violet_web", Color::new(238, 130, 238, 255));
        m.insert("viridian", Color::new(64, 130, 109, 255));
        m.insert("vivid_auburn", Color::new(146, 39, 36, 255));
        m.insert("vivid_burgundy", Color::new(159, 29, 53, 255));
        m.insert("vivid_cerise", Color::new(218, 29, 129, 255));
        m.insert("vivid_tangerine", Color::new(255, 160, 137, 255));
        m.insert("vivid_violet", Color::new(159, 0, 255, 255));
        m.insert("warm_black", Color::new(0, 66, 66, 255));
        m.insert("waterspout", Color::new(164, 244, 249, 255));
        m.insert("wenge", Color::new(100, 84, 82, 255));
        m.insert("wheat", Color::new(245, 222, 179, 255));
        m.insert("white", Color::new(255, 255, 255, 255));
        m.insert("white_smoke", Color::new(245, 245, 245, 255));
        m.insert("wild_blue_yonder", Color::new(162, 173, 208, 255));
        m.insert("wild_strawberry", Color::new(255, 67, 164, 255));
        m.insert("wild_watermelon", Color::new(252, 108, 133, 255));
        m.insert("wine", Color::new(114, 47, 55, 255));
        m.insert("wine_dregs", Color::new(103, 49, 71, 255));
        m.insert("wisteria", Color::new(201, 160, 220, 255));
        m.insert("wood_brown", Color::new(193, 154, 107, 255));
        m.insert("xanadu", Color::new(115, 134, 120, 255));
        m.insert("yale_blue", Color::new(15, 77, 146, 255));
        m.insert("yellow", Color::new(255, 255, 0, 255));
        m.insert("yellow_green", Color::new(154, 205, 50, 255));
        m.insert("yellow_munsell", Color::new(239, 204, 0, 255));
        m.insert("yellow_ncs", Color::new(255, 211, 0, 255));
        m.insert("yellow_orange", Color::new(255, 174, 66, 255));
        m.insert("yellow_process", Color::new(255, 239, 0, 255));
        m.insert("yellow_ryb", Color::new(254, 254, 51, 255));
        m.insert("zaffre", Color::new(0, 20, 168, 255));
        m.insert("zinnwaldite_brown", Color::new(44, 22, 8, 255));

        m
    };
}
