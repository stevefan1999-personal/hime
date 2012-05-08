﻿/*
 * WARNING: this file has been generated by
 * Hime Parser Generator 0.4.0.0
 */

using System.Collections.Generic;
using Hime.Redist.Parsers;

namespace Hime.Parsers.Input
{
    public sealed class FileCentralDogmaLexer : LexerText
    {
        public static readonly SymbolTerminal[] terminals = {
            new SymbolTerminal(0x1, "ε"),
            new SymbolTerminal(0x2, "$"),
            new SymbolTerminal(0x93, "["),
            new SymbolTerminal(0x13, "INTEGER"),
            new SymbolTerminal(0x1C, "="),
            new SymbolTerminal(0x1D, ";"),
            new SymbolTerminal(0x1E, "."),
            new SymbolTerminal(0x20, "("),
            new SymbolTerminal(0x21, ")"),
            new SymbolTerminal(0x22, "*"),
            new SymbolTerminal(0x23, "+"),
            new SymbolTerminal(0x24, "?"),
            new SymbolTerminal(0x25, "{"),
            new SymbolTerminal(0x26, ","),
            new SymbolTerminal(0x27, "}"),
            new SymbolTerminal(0x28, "-"),
            new SymbolTerminal(0x29, "|"),
            new SymbolTerminal(0x2C, "<"),
            new SymbolTerminal(0x2D, ">"),
            new SymbolTerminal(0x2E, "^"),
            new SymbolTerminal(0x2F, "!"),
            new SymbolTerminal(0xA, "NAME"),
            new SymbolTerminal(0x33, ":"),
            new SymbolTerminal(0x94, "]"),
            new SymbolTerminal(0x7, "SEPARATOR"),
            new SymbolTerminal(0x14, "QUOTED_DATA"),
            new SymbolTerminal(0x15, "ESCAPEES"),
            new SymbolTerminal(0x2A, "=>"),
            new SymbolTerminal(0x1F, ".."),
            new SymbolTerminal(0x2B, "->"),
            new SymbolTerminal(0x34, "cf"),
            new SymbolTerminal(0x95, "cs"),
            new SymbolTerminal(0x16, "SYMBOL_TERMINAL_TEXT"),
            new SymbolTerminal(0x17, "SYMBOL_TERMINAL_SET"),
            new SymbolTerminal(0x1A, "SYMBOL_VALUE_UINT8"),
            new SymbolTerminal(0x32, "rules"),
            new SymbolTerminal(0x18, "SYMBOL_TERMINAL_UBLOCK"),
            new SymbolTerminal(0x19, "SYMBOL_TERMINAL_UCAT"),
            new SymbolTerminal(0x1B, "SYMBOL_VALUE_UINT16"),
            new SymbolTerminal(0x30, "options"),
            new SymbolTerminal(0x35, "grammar"),
            new SymbolTerminal(0x31, "terminals") };
        private static LexerDFAState[] staticStates = { 
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x1 },
                new ushort[3] { 0x22, 0x22, 0x3 },
                new ushort[3] { 0x27, 0x27, 0x4 },
                new ushort[3] { 0x5B, 0x5B, 0x1F },
                new ushort[3] { 0x5C, 0x5C, 0x5 },
                new ushort[3] { 0x30, 0x30, 0x20 },
                new ushort[3] { 0x3D, 0x3D, 0x22 },
                new ushort[3] { 0x3B, 0x3B, 0x23 },
                new ushort[3] { 0x2E, 0x2E, 0x24 },
                new ushort[3] { 0x28, 0x28, 0x25 },
                new ushort[3] { 0x29, 0x29, 0x26 },
                new ushort[3] { 0x2A, 0x2A, 0x27 },
                new ushort[3] { 0x2B, 0x2B, 0x28 },
                new ushort[3] { 0x3F, 0x3F, 0x29 },
                new ushort[3] { 0x7B, 0x7B, 0x2A },
                new ushort[3] { 0x2C, 0x2C, 0x2B },
                new ushort[3] { 0x7D, 0x7D, 0x2C },
                new ushort[3] { 0x2D, 0x2D, 0x2D },
                new ushort[3] { 0x7C, 0x7C, 0x2E },
                new ushort[3] { 0x3C, 0x3C, 0x2F },
                new ushort[3] { 0x3E, 0x3E, 0x30 },
                new ushort[3] { 0x5E, 0x5E, 0x31 },
                new ushort[3] { 0x21, 0x21, 0x32 },
                new ushort[3] { 0x6F, 0x6F, 0x33 },
                new ushort[3] { 0x74, 0x74, 0x34 },
                new ushort[3] { 0x72, 0x72, 0x35 },
                new ushort[3] { 0x3A, 0x3A, 0x39 },
                new ushort[3] { 0x63, 0x63, 0x36 },
                new ushort[3] { 0x67, 0x67, 0x37 },
                new ushort[3] { 0x5D, 0x5D, 0x3A },
                new ushort[3] { 0xA, 0xA, 0x3B },
                new ushort[3] { 0x2028, 0x2029, 0x3B },
                new ushort[3] { 0x9, 0x9, 0x3D },
                new ushort[3] { 0xB, 0xC, 0x3D },
                new ushort[3] { 0x20, 0x20, 0x3D },
                new ushort[3] { 0x41, 0x5A, 0x38 },
                new ushort[3] { 0x5F, 0x5F, 0x38 },
                new ushort[3] { 0x61, 0x62, 0x38 },
                new ushort[3] { 0x64, 0x66, 0x38 },
                new ushort[3] { 0x68, 0x6E, 0x38 },
                new ushort[3] { 0x70, 0x71, 0x38 },
                new ushort[3] { 0x73, 0x73, 0x38 },
                new ushort[3] { 0x75, 0x7A, 0x38 },
                new ushort[3] { 0x370, 0x3FF, 0x38 },
                new ushort[3] { 0x31, 0x39, 0x21 },
                new ushort[3] { 0x40, 0x40, 0x6 },
                new ushort[3] { 0xD, 0xD, 0x3C }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x7 },
                new ushort[3] { 0x2A, 0x2A, 0x8 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x9 },
                new ushort[3] { 0x2F, 0x2F, 0x16 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x0, 0x21, 0x3 },
                new ushort[3] { 0x23, 0xFFFF, 0x3 },
                new ushort[3] { 0x22, 0x22, 0x3E }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x5C, 0x5C, 0xA },
                new ushort[3] { 0x0, 0x26, 0xB },
                new ushort[3] { 0x28, 0x5B, 0xB },
                new ushort[3] { 0x5D, 0xFFFF, 0xB }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x75, 0x75, 0xE },
                new ushort[3] { 0x30, 0x30, 0x3F },
                new ushort[3] { 0x5C, 0x5C, 0x3F },
                new ushort[3] { 0x61, 0x62, 0x3F },
                new ushort[3] { 0x66, 0x66, 0x3F },
                new ushort[3] { 0x6E, 0x6E, 0x3F },
                new ushort[3] { 0x72, 0x72, 0x3F },
                new ushort[3] { 0x74, 0x74, 0x3F },
                new ushort[3] { 0x76, 0x76, 0x3F }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x41, 0x5A, 0x48 },
                new ushort[3] { 0x5F, 0x5F, 0x48 },
                new ushort[3] { 0x61, 0x7A, 0x48 },
                new ushort[3] { 0x370, 0x3FF, 0x48 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x0, 0x9, 0x7 },
                new ushort[3] { 0xB, 0xC, 0x7 },
                new ushort[3] { 0xE, 0x2027, 0x7 },
                new ushort[3] { 0x202A, 0xFFFF, 0x7 },
                new ushort[3] { 0xA, 0xA, 0x5D },
                new ushort[3] { 0x2028, 0x2029, 0x5D },
                new ushort[3] { 0xD, 0xD, 0x5E }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x10 },
                new ushort[3] { 0x0, 0x29, 0x11 },
                new ushort[3] { 0x2B, 0xFFFF, 0x11 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x19 },
                new ushort[3] { 0x0, 0x29, 0x12 },
                new ushort[3] { 0x2B, 0xFFFF, 0x12 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x27, 0x27, 0xB },
                new ushort[3] { 0x30, 0x30, 0xB },
                new ushort[3] { 0x5C, 0x5C, 0xB },
                new ushort[3] { 0x61, 0x62, 0xB },
                new ushort[3] { 0x66, 0x66, 0xB },
                new ushort[3] { 0x6E, 0x6E, 0xB },
                new ushort[3] { 0x72, 0x72, 0xB },
                new ushort[3] { 0x74, 0x74, 0xB },
                new ushort[3] { 0x76, 0x76, 0xB }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x27, 0x27, 0x5F },
                new ushort[3] { 0x5C, 0x5C, 0xA },
                new ushort[3] { 0x0, 0x26, 0xB },
                new ushort[3] { 0x28, 0x5B, 0xB },
                new ushort[3] { 0x5D, 0xFFFF, 0xB }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x30, 0xD },
                new ushort[3] { 0x5B, 0x5D, 0xD },
                new ushort[3] { 0x61, 0x62, 0xD },
                new ushort[3] { 0x66, 0x66, 0xD },
                new ushort[3] { 0x6E, 0x6E, 0xD },
                new ushort[3] { 0x72, 0x72, 0xD },
                new ushort[3] { 0x74, 0x74, 0xD },
                new ushort[3] { 0x76, 0x76, 0xD }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x5D, 0x5D, 0x60 },
                new ushort[3] { 0x5C, 0x5C, 0xC },
                new ushort[3] { 0x0, 0x5A, 0xD },
                new ushort[3] { 0x5E, 0xFFFF, 0xD }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x62, 0x62, 0x13 },
                new ushort[3] { 0x63, 0x63, 0x14 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x15 },
                new ushort[3] { 0x41, 0x46, 0x15 },
                new ushort[3] { 0x61, 0x66, 0x15 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x10 },
                new ushort[3] { 0x0, 0x29, 0x11 },
                new ushort[3] { 0x2B, 0x2E, 0x11 },
                new ushort[3] { 0x30, 0xFFFF, 0x11 },
                new ushort[3] { 0x2F, 0x2F, 0x61 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x10 },
                new ushort[3] { 0x0, 0x29, 0x11 },
                new ushort[3] { 0x2B, 0xFFFF, 0x11 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x19 },
                new ushort[3] { 0x0, 0x29, 0x12 },
                new ushort[3] { 0x2B, 0xFFFF, 0x12 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x7B, 0x7B, 0x17 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x7B, 0x7B, 0x18 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x62 },
                new ushort[3] { 0x41, 0x46, 0x62 },
                new ushort[3] { 0x61, 0x66, 0x62 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x0, 0x9, 0x16 },
                new ushort[3] { 0xB, 0xC, 0x16 },
                new ushort[3] { 0xE, 0x2027, 0x16 },
                new ushort[3] { 0x202A, 0xFFFF, 0x16 },
                new ushort[3] { 0xA, 0xA, 0x5B },
                new ushort[3] { 0xD, 0xD, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x40, 0x40, 0x1A },
                new ushort[3] { 0x41, 0x5A, 0x1C },
                new ushort[3] { 0x5F, 0x5F, 0x1C },
                new ushort[3] { 0x61, 0x7A, 0x1C },
                new ushort[3] { 0x370, 0x3FF, 0x1C }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x40, 0x40, 0x1B },
                new ushort[3] { 0x41, 0x5A, 0x1D },
                new ushort[3] { 0x5F, 0x5F, 0x1D },
                new ushort[3] { 0x61, 0x7A, 0x1D },
                new ushort[3] { 0x370, 0x3FF, 0x1D }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2A, 0x2A, 0x19 },
                new ushort[3] { 0x0, 0x29, 0x12 },
                new ushort[3] { 0x2B, 0x2E, 0x12 },
                new ushort[3] { 0x30, 0xFFFF, 0x12 },
                new ushort[3] { 0x2F, 0x2F, 0x5B }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x41, 0x5A, 0x1C },
                new ushort[3] { 0x5F, 0x5F, 0x1C },
                new ushort[3] { 0x61, 0x7A, 0x1C },
                new ushort[3] { 0x370, 0x3FF, 0x1C }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x41, 0x5A, 0x1D },
                new ushort[3] { 0x5F, 0x5F, 0x1D },
                new ushort[3] { 0x61, 0x7A, 0x1D },
                new ushort[3] { 0x370, 0x3FF, 0x1D }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x1C },
                new ushort[3] { 0x41, 0x5A, 0x1C },
                new ushort[3] { 0x5F, 0x5F, 0x1C },
                new ushort[3] { 0x61, 0x7A, 0x1C },
                new ushort[3] { 0x370, 0x3FF, 0x1C },
                new ushort[3] { 0x7D, 0x7D, 0x64 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x1D },
                new ushort[3] { 0x41, 0x5A, 0x1D },
                new ushort[3] { 0x5F, 0x5F, 0x1D },
                new ushort[3] { 0x61, 0x7A, 0x1D },
                new ushort[3] { 0x370, 0x3FF, 0x1D },
                new ushort[3] { 0x7D, 0x7D, 0x65 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x66 },
                new ushort[3] { 0x41, 0x46, 0x66 },
                new ushort[3] { 0x61, 0x66, 0x66 }},
                null),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x5C, 0x5C, 0xC },
                new ushort[3] { 0x0, 0x5A, 0xD },
                new ushort[3] { 0x5E, 0xFFFF, 0xD }},
                terminals[0x2]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x78, 0x78, 0xF }},
                terminals[0x3]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x5C }},
                terminals[0x3]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x3E, 0x3E, 0x40 }},
                terminals[0x4]),
            new LexerDFAState(new ushort[][] {}, terminals[0x5]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2E, 0x2E, 0x41 }},
                terminals[0x6]),
            new LexerDFAState(new ushort[][] {}, terminals[0x7]),
            new LexerDFAState(new ushort[][] {}, terminals[0x8]),
            new LexerDFAState(new ushort[][] {}, terminals[0x9]),
            new LexerDFAState(new ushort[][] {}, terminals[0xA]),
            new LexerDFAState(new ushort[][] {}, terminals[0xB]),
            new LexerDFAState(new ushort[][] {}, terminals[0xC]),
            new LexerDFAState(new ushort[][] {}, terminals[0xD]),
            new LexerDFAState(new ushort[][] {}, terminals[0xE]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x3E, 0x3E, 0x42 }},
                terminals[0xF]),
            new LexerDFAState(new ushort[][] {}, terminals[0x10]),
            new LexerDFAState(new ushort[][] {}, terminals[0x11]),
            new LexerDFAState(new ushort[][] {}, terminals[0x12]),
            new LexerDFAState(new ushort[][] {}, terminals[0x13]),
            new LexerDFAState(new ushort[][] {}, terminals[0x14]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x70, 0x70, 0x43 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6F, 0x44 },
                new ushort[3] { 0x71, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x65, 0x65, 0x45 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x64, 0x44 },
                new ushort[3] { 0x66, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x75, 0x75, 0x46 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x74, 0x44 },
                new ushort[3] { 0x76, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x66, 0x66, 0x59 },
                new ushort[3] { 0x73, 0x73, 0x5A },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x65, 0x44 },
                new ushort[3] { 0x67, 0x72, 0x44 },
                new ushort[3] { 0x74, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x72, 0x72, 0x47 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x71, 0x44 },
                new ushort[3] { 0x73, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {}, terminals[0x16]),
            new LexerDFAState(new ushort[][] {}, terminals[0x17]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0xA, 0xA, 0x3B },
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0x9, 0x5B },
                new ushort[3] { 0xB, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {}, terminals[0x19]),
            new LexerDFAState(new ushort[][] {}, terminals[0x1A]),
            new LexerDFAState(new ushort[][] {}, terminals[0x1B]),
            new LexerDFAState(new ushort[][] {}, terminals[0x1C]),
            new LexerDFAState(new ushort[][] {}, terminals[0x1D]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x74, 0x74, 0x49 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x73, 0x44 },
                new ushort[3] { 0x75, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x72, 0x72, 0x4A },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x71, 0x44 },
                new ushort[3] { 0x73, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6C, 0x6C, 0x4C },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6B, 0x44 },
                new ushort[3] { 0x6D, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x61, 0x61, 0x4B },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x62, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x69, 0x69, 0x4D },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x68, 0x44 },
                new ushort[3] { 0x6A, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6D, 0x6D, 0x4E },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6C, 0x44 },
                new ushort[3] { 0x6E, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6D, 0x6D, 0x4F },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6C, 0x44 },
                new ushort[3] { 0x6E, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x65, 0x65, 0x55 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x64, 0x44 },
                new ushort[3] { 0x66, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6F, 0x6F, 0x50 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6E, 0x44 },
                new ushort[3] { 0x70, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x69, 0x69, 0x51 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x68, 0x44 },
                new ushort[3] { 0x6A, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6D, 0x6D, 0x52 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6C, 0x44 },
                new ushort[3] { 0x6E, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6E, 0x6E, 0x56 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6D, 0x44 },
                new ushort[3] { 0x6F, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6E, 0x6E, 0x53 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6D, 0x44 },
                new ushort[3] { 0x6F, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x61, 0x61, 0x57 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x62, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x61, 0x61, 0x54 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x62, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x6C, 0x6C, 0x58 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x6B, 0x44 },
                new ushort[3] { 0x6D, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x73, 0x73, 0x63 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x72, 0x44 },
                new ushort[3] { 0x74, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x73, 0x73, 0x67 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x72, 0x44 },
                new ushort[3] { 0x74, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x72, 0x72, 0x68 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x71, 0x44 },
                new ushort[3] { 0x73, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x73, 0x73, 0x69 },
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x72, 0x44 },
                new ushort[3] { 0x74, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x15]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x1E]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x1F]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x5C }},
                terminals[0x3]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0xA, 0xA, 0x5D },
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0x9, 0x5B },
                new ushort[3] { 0xB, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {}, terminals[0x20]),
            new LexerDFAState(new ushort[][] {}, terminals[0x21]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x2F, 0x2F, 0x2 },
                new ushort[3] { 0x9, 0xD, 0x5B },
                new ushort[3] { 0x20, 0x20, 0x5B },
                new ushort[3] { 0x2028, 0x2029, 0x5B }},
                terminals[0x18]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x1E },
                new ushort[3] { 0x41, 0x46, 0x1E },
                new ushort[3] { 0x61, 0x66, 0x1E }},
                terminals[0x22]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x23]),
            new LexerDFAState(new ushort[][] {}, terminals[0x24]),
            new LexerDFAState(new ushort[][] {}, terminals[0x25]),
            new LexerDFAState(new ushort[][] {}, terminals[0x26]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x27]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x28]),
            new LexerDFAState(new ushort[][] {
                new ushort[3] { 0x30, 0x39, 0x44 },
                new ushort[3] { 0x41, 0x5A, 0x44 },
                new ushort[3] { 0x5F, 0x5F, 0x44 },
                new ushort[3] { 0x61, 0x7A, 0x44 },
                new ushort[3] { 0x370, 0x3FF, 0x44 }},
                terminals[0x29]) };
        protected override void setup() {
            states = staticStates;
            subGrammars = new Dictionary<ushort, MatchSubGrammar>();
            separatorID = 0x7;
        }
        public override LexerText Clone() {
            return new FileCentralDogmaLexer(this);
        }
        public FileCentralDogmaLexer(string input) : base(new System.IO.StringReader(input)) {}
        public FileCentralDogmaLexer(System.IO.TextReader input) : base(input) {}
        public FileCentralDogmaLexer(FileCentralDogmaLexer original) : base(original) {}
    }
}
