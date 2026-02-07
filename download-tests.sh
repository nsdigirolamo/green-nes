#!/bin/bash

function download_test_file() {
    path="$1"
    url="$2"
    expected_hash="$3"

    if [[ -e "$path" ]]; then
        echo -e "\e[37mSkipping download (already exists): $path\e[0m"
    else
        curl --create-dirs --output $path $url
    fi

    actual_hash="$(sha256sum $path)"

    if [[ "$expected_hash" != "$actual_hash" ]]; then
        echo -e "\e[31mHash check failed: \e[0m\e[33m$path\e[0m\e[31m from \e[0\e[33m$url\e[0m"
        echo -e "\e[31mExpected:\e[0m $expected_hash"
        echo -e "\e[31mActual:\e[0m   $actual_hash"
        rm "$path"
    fi
}

echo ""

NES_TEST_PATH="./tests/nestest"

NES_TEST_ROM_PATH="$NES_TEST_PATH/nestest.nes"
NES_TEST_ROM_URL="https://nickmass.com/images/nestest.nes"
NES_TEST_ROM_HASH="f67d55fd6b3cf0bad1cc85f1df0d739c65b53e79cecb7fea8f77ec0eadab0004  ./tests/nestest/nestest.nes"
download_test_file "$NES_TEST_ROM_PATH" "$NES_TEST_ROM_URL" "$NES_TEST_ROM_HASH"

NES_TEST_LOG_PATH="$NES_TEST_PATH/nestest.log"
NES_TEST_LOG_URL="https://www.qmtpro.com/~nes/misc/nestest.log"
NES_TEST_LOG_HASH="627c8e180b1a924dfa705c5dc6958fad7ab75a62de556173caf880ccc1337540  ./tests/nestest/nestest.log"
download_test_file "$NES_TEST_LOG_PATH" "$NES_TEST_LOG_URL" "$NES_TEST_LOG_HASH"

NES_TEST_DOC_PATH="$NES_TEST_PATH/nestest.txt"
NES_TEST_DOC_URL="https://www.qmtpro.com/~nes/misc/nestest.txt"
NES_TEST_DOC_HASH="8291241ba9a0885b9a604a4685101a1473e22b3aa070bc828e3b8c342d7f71fb  ./tests/nestest/nestest.txt"
download_test_file "$NES_TEST_DOC_PATH" "$NES_TEST_DOC_URL" "$NES_TEST_DOC_HASH"

echo ""
