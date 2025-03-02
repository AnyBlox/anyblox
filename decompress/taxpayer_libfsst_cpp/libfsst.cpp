// this software is distributed under the MIT License
// (http://www.opensource.org/licenses/MIT):
//
// Copyright 2018-2020, CWI, TU Munich, FSU Jena
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// - The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// You can contact the authors via the FSST source repository :
// https://github.com/cwida/fsst
#include "libfsst.hpp"

#define FSST_CORRUPT                                                           \
  32774747032022883 /* 7-byte number in little endian containing "corrupt" */

extern "C" u32 fsst_import(fsst_decoder_t *decoder, const u8 *buf) {
  u64 version = 0;
  u32 code, pos = 17;
  u8 lenHisto[8];

  // version field (first 8 bytes) is now there just for future-proofness,
  // unused still (skipped)
  memcpy(&version, buf, 8);
  if ((version >> 32) != FSST_VERSION)
    return 0;
  decoder->zeroTerminated = buf[8] & 1;
  memcpy(lenHisto, buf + 9, 8);

  // in case of zero-terminated, first symbol is "" (zero always, may be
  // overwritten)
  decoder->len[0] = 1;
  decoder->symbol[0] = 0;

  // we use lenHisto[0] as 1-byte symbol run length (at the end)
  code = decoder->zeroTerminated;
  if (decoder->zeroTerminated)
    lenHisto[0]--; // if zeroTerminated, then symbol "" aka 1-byte code=0, is
                   // not stored at the end

  // now get all symbols from the buffer
  for (u32 l = 1; l <= 8; l++) { /* l = 1,2,3,4,5,6,7,8 */
    for (u32 i = 0; i < lenHisto[(l & 7) /* 1,2,3,4,5,6,7,0 */]; i++, code++) {
      decoder->len[code] = (l & 7) + 1; /* len = 2,3,4,5,6,7,8,1  */
      decoder->symbol[code] = 0;
      for (u32 j = 0; j < decoder->len[code]; j++)
        ((u8 *)&decoder->symbol[code])[j] =
            buf[pos++]; // note this enforces 'little endian' symbols
    }
  }
  if (decoder->zeroTerminated)
    lenHisto[0]++;

  // fill unused symbols with text "corrupt". Gives a chance to detect corrupted
  // code sequences (if there are unused symbols).
  while (code < 255) {
    decoder->symbol[code] = FSST_CORRUPT;
    decoder->len[code++] = 8;
  }
  return pos;
}
