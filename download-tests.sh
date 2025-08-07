#!/bin/bash

function download_test_file() {
    path="$1"
    url="$2"
    expected_hash="$3"

    if [[ -e "$path" ]]; then
        echo -e "\e[37m$path already exists. Skipping download...\e[0m"
    else
        curl --create-dirs --output $path $url
    fi

    actual_hash="$(sha256sum $path)"

    if [[ "$expected_hash" != "$actual_hash" ]]; then
        echo -e "\e[31mHash check failed: \e[0m\e[33m$path\e[0m\e[31m from \e[0\e[33m$url\e[0m"
        rm "$path"
    fi
}

NES_TEST_PATH="./tests/nestest.nes"
NES_TEST_URL="https://nickmass.com/images/nestest.nes"
NES_TEST_HASH="f67d55fd6b3cf0bad1cc85f1df0d739c65b53e79cecb7fea8f77ec0eadab0004  ./tests/nestest.nes"
download_test_file "$NES_TEST_PATH" "$NES_TEST_URL" "$NES_TEST_HASH"

NES_TEST_LOG_PATH="./tests/nestest.log"
NES_TEST_LOG_URL="https://www.qmtpro.com/~nes/misc/nestest.log"
NES_TEST_LOG_HASH="627c8e180b1a924dfa705c5dc6958fad7ab75a62de556173caf880ccc1337540  ./tests/nestest.log"
download_test_file "$NES_TEST_LOG_PATH" "$NES_TEST_LOG_URL" "$NES_TEST_LOG_HASH"
