#[allow(bad_style)]
#[derive(Debug, Copy, Clone)]
pub enum Pin {
  GPIO_P8_3 = 38,
  GPIO_P8_4 = 39,
  GPIO_P8_5 = 34,
  GPIO_P8_6 = 35,
  GPIO_P8_7 = 66,
  GPIO_P8_8 = 67,
  GPIO_P8_9 = 69,
  GPIO_P8_10 = 68,
  GPIO_P8_11 = 45,
  GPIO_P8_12 = 44,
  GPIO_P8_13 = 23,
  GPIO_P8_14 = 26,
  GPIO_P8_15 = 47,
  GPIO_P8_16 = 46,
  GPIO_P8_17 = 27,
  GPIO_P8_18 = 65,
  GPIO_P8_19 = 22,
  GPIO_P8_20 = 63,
  GPIO_P8_21 = 62,
  GPIO_P8_22 = 37,
  GPIO_P8_23 = 36,
  GPIO_P8_24 = 33,
  GPIO_P8_25 = 32,
  GPIO_P8_26 = 61,
  GPIO_P8_27 = 86,
  GPIO_P8_28 = 88,
  GPIO_P8_29 = 87,
  GPIO_P8_30 = 89,
  GPIO_P8_31 = 10,
  GPIO_P8_32 = 11,
  GPIO_P8_33 = 9,
  GPIO_P8_34 = 81,
  GPIO_P8_35 = 8,
  GPIO_P8_36 = 80,
  GPIO_P8_37 = 78,
  GPIO_P8_38 = 79,
  GPIO_P8_39 = 76,
  GPIO_P8_40 = 77,
  GPIO_P8_41 = 74,
  GPIO_P8_42 = 75,
  GPIO_P8_43 = 72,
  GPIO_P8_44 = 73,
  GPIO_P8_45 = 70,
  GPIO_P8_46 = 71,
  GPIO_P9_11 = 30,
  GPIO_P9_12 = 60,
  GPIO_P9_13 = 31,
  GPIO_P9_14 = 40,
  GPIO_P9_15 = 48,
  GPIO_P9_16 = 51,
  GPIO_P9_17 = 4,
  GPIO_P9_18 = 5,
  GPIO_P9_21 = 3,
  GPIO_P9_22 = 2,
  GPIO_P9_23 = 49,
  GPIO_P9_24 = 15,
  GPIO_P9_25 = 117,
  GPIO_P9_26 = 14,
  GPIO_P9_27 = 125,
  GPIO_P9_28 = 123,
  GPIO_P9_29 = 121,
  GPIO_P9_30 = 122,
  GPIO_P9_31 = 120,
  GPIO_P9_41 = 20,
  GPIO_P9_42 = 7,

  // This is the hackiest of hacks.
  // To avoid having enum variants point to the same value, we just increase all of the ADC
  // variants by 1000 (and then subtract them later in the code).
  // Yeah, I'm not too proud of this one.
  AIN_0 = 1000,
  AIN_1 = 1001,
  AIN_2 = 1002,
  AIN_3 = 1003,
  AIN_4 = 1004,
  AIN_5 = 1005,
  AIN_6 = 1006,
  AIN_7 = 1007,

  // Unfortunately it seems like the pin aliases change depending on which cape is loaded,
  // meaning we'd have to implement a way to adjust the aliases.
  // That will have to wait for now.
  // See link below for some more details.
  // https://groups.google.com/d/msg/beagleboard/1mkf_s_g0vI/55aA84qNAQAJ

  // 0  EHRPWM0A  P9.22,P9.31
  // 1  EHRPWM0B  P9.21,P9.29
  // 2  ECAPPWM0  P9.42
  // 3  EHRPWM1A  P9.14,P8.36
  // 4  EHRPWM1B  P9.16,P8.34
  // 5  EHRPWM2A  P8.19,P8.45
  // 6  EHRPWM2B  P8.13,P8.46
  // 7  ECAPPWM2  P9.28

  // PWM_P = (0,0),
  // PWM_P = (0,1),
  // PWM_P = (2,0),
  // PWM_P = (2,1),
  // PWM_P = (4,0),
  // PWM_P = (4,1),
}
