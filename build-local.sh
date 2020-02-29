#!/bin/bash

# Copyright 2019 Walmart Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

function fail() {
  echo "[Error]: ${*}"
}

# If the first argument is force, then get the file again
force=${1}

if [[ ! -d "sawtooth-sabre" || ${force} == "force" ]]; then
  git clone https://github.com/arsulegai/sawtooth-sabre
fi

cd "sawtooth-sabre" || fail "Unable to find the sawtooth-sabre"
# git checkout origin/add_event
docker-compose -f docker-compose-installed.yaml build sabre-tp
cd - || fail "Cannot find the parent directory after sawtooth-sabre"
