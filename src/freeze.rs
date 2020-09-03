// Copyright 2020 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// 当数据被不可变地借用时，它还会冻结（freeze）。
// 已冻结的数据无法通过原始 对象来修改，直到对这些数据的所有引用离开作用域为止。

pub fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let large_integer = &_mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉此行

        println!("Immutably borrowed {}", large_integer);

        // `large_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_integer = 3;
    println!("Mutable Integer now is {}", _mutable_integer);
}
