mod front_of_house;

pb use crate::font_of_house::hosting;

pb fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
