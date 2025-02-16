#!/bin/bash
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
set -x

# This bash script is meant to be run inside the docker-compose environment. Check the README for instructions

#TODO: add queries 15, 16, 19, 20, and 22 once we support them

for query in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 17 18 21
do
  /root/tpch benchmark ballista --host ballista-scheduler --port 50050 --query $query --path /data --format tbl --iterations 1 --debug
done
