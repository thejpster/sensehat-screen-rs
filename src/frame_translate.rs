//! Frame scrolling for the LED Matrix screen

#[cfg(test)]
mod tests {
    use super::*;

    const DARK: PixelColor = PixelColor::BLACK;
    const BLUE: PixelColor = PixelColor::BLUE;

    const FRAME_ZERO: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
    ];

    const SYMBOL_FRAME: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    // RIGHT
    const OFFSET_RIGHT_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, BLUE, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_FIVE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SIX: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SEVEN: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
    ];

    // LEFT
    const OFFSET_LEFT_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, DARK,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, DARK,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
    ];

    const OFFSET_LEFT_TWO: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
    ];

    const OFFSET_LEFT_THREE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_FIVE: [PixelColor; 64] = [
        DARK, DARK, BLUE, DARK, DARK, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_SIX: [PixelColor; 64] = [
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_SEVEN: [PixelColor; 64] = [
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    // BOTTOM
    const OFFSET_TOP_ONE: [PixelColor; 64] = [
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FOUR: [PixelColor; 64] = [
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FIVE: [PixelColor; 64] = [
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_SIX: [PixelColor; 64] = [
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_SEVEN: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    // BOTTOM
    const OFFSET_BOTTOM_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
    ];

    const OFFSET_BOTTOM_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FOUR: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FIVE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SIX: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SEVEN: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
    ];

}
