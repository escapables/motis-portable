#include "native/base64.h"

namespace motis::native {

namespace {

constexpr auto kBase64Chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

}  // namespace

std::string encode_base64(std::string const& input) {
  std::string encoded;
  int i = 0;
  unsigned char char_array_3[3];
  unsigned char char_array_4[4];

  for (size_t in_len = input.size(), pos = 0; in_len--;) {
    char_array_3[i++] = input[pos++];
    if (i == 3) {
      char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
      char_array_4[1] =
          ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
      char_array_4[2] =
          ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
      char_array_4[3] = char_array_3[2] & 0x3f;

      for (auto j = 0; j < 4; ++j) {
        encoded += kBase64Chars[char_array_4[j]];
      }
      i = 0;
    }
  }

  if (i) {
    for (auto j = i; j < 3; ++j) {
      char_array_3[j] = '\0';
    }

    char_array_4[0] = (char_array_3[0] & 0xfc) >> 2;
    char_array_4[1] =
        ((char_array_3[0] & 0x03) << 4) + ((char_array_3[1] & 0xf0) >> 4);
    char_array_4[2] =
        ((char_array_3[1] & 0x0f) << 2) + ((char_array_3[2] & 0xc0) >> 6);
    char_array_4[3] = char_array_3[2] & 0x3f;

    for (auto j = 0; j < (i + 1); ++j) {
      encoded += kBase64Chars[char_array_4[j]];
    }

    while (i < 3) {
      ++i;
      encoded += '=';
    }
  }

  return encoded;
}

}  // namespace motis::native
