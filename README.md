A simple game solver in Rust to make me less terrible at Chinese Checkers. The board state is represented by a 128-bit number where 0 is empty, 1 is full, and each binary digit corresponds to a location on the board:

                    00
                  02  01
                05  04  03
              09  08  07  06
            14  13  12  11  10
          20  19  18  17  16  15
        27  26  25  24  23  22  21
      35  34  33  32  31  30  29  28
    44  43  42  41  40  39  38  37  36
      52  51  50  49  48  47  46  45
        59  58  57  56  55  54  53
          65  64  63  62  61  60
            70  69  68  67  66
              74  73  72  71
                77  76  75
                  79  78
                    80
