let input_str =
    `1	2	s21
2	3	s22
3	4	s12
4	5	s11
5	6	s11
6	7	s11
2	4	s11
3	5	s11
4	3	s11
6	5	s11
6	2	s11`


let adj_list = parseAdjacencyList(input_str);
let res = parsePathAndCycle(adj_list, 1, 7);
console.log(`前向通路：`);
console.log(`   ${JSON.stringify(Array.from(res.path.paths))}`);
res = calcCycle(res.cycle);
console.log(`所有环路：`);
console.log(`   ${JSON.stringify(res[0])}`);
console.log(`所有两两不相邻环路：`);
console.log(`   ${JSON.stringify(res[1])}`);

// 1. 解析文本，返回邻接表。其实是[{i:int, o: int, g:string}, ...]
function parseAdjacencyList(text) {
    return text.split(/\r\n|\n/).map(line => {
        let node = line.trim().split(/\s+/);
        if (node.length < 3) {
            throw "accept 3 params only"
        }
        return { i: parseInt(node[0]), o: parseInt(node[1]), g: node[2] }
    }).sort((elem1, elem2) => {
        let temp = elem1.o - elem2.o;
        return (elem1.i - elem2.i) * (Math.abs(temp) + 1) + temp;
    });
}

// 2. 获取邻接表中前向通道与闭环的信息
function parsePathAndCycle(adjaccency_list, input, output) {
    let stack = [];
    stack.push(input)
    let cs = new CycleStack(),
        ps = new PathStack();
    // 自动执行的递归函数
    (function DFS() {
        let nodes = adj_list.filter(el => { return el.i === stack[stack.length - 1] });

        nodes.map(el => {
            let temp = el.o;
            stack.push(temp);
            if (temp === output) {
                ps.push(stack.slice());
            } else if (stack.slice(0, -1).indexOf(temp) > -1) {
                let cycle = stack.slice(stack.indexOf(temp));
                cs.push(cycle)
            } else {
                DFS();
            }
            stack.pop();
        });
    })();

    // 移除信号不会反馈到前向通道的环路
    cs.cycels.forEach((v, k) => {
        if (!ps.nodes.has(v[0])) {
            cs.cycels.delete(k)
        }
    })

    return { path: ps, cycle: cs }
}

// 3. 去除数组中的重复元素，所有对象都会被JSON.stringfy() 之后再做比较
function Distinct(arr) {
    let res = [];
    let obj = {};
    for (let i of arr) {
        if (!obj[i]) {
            res.push(i);
            obj[i] = 1;
        }
    }
    return res.sort();
}

// 4. 不重复的回路集合
function CycleStack() {
    this.cycels = new Map();
    this.push = cycle => {
        let str = Distinct(cycle).join("");
        // console.log(`cycle: ${str}`);
        if (!this.cycels.get(str)) {
            this.cycels.set(str, cycle);
            // console.log(`${cycle.join('-')}`);
        }
    }
    this.toArray = () => {
        return Array.from(this.cycels.values())
    }
    this.size = () => {
        return this.cycels.size;
    }
    return this;
}

// 5. 前向通道集合
function PathStack() {
    this.paths = new Set();
    this.nodes = new Set();

    this.push = path => {
        this.paths.add(path)
        path.map(i => {
            this.nodes.add(i)
        })
        // console.log(`${path.join('-')}`);
    }

    return this;
}

// 6. 查找不相交的环路
// 只有n-n 不相交的回路才有可能组成(n+1)-(n+1)不相交的回路
// 所以上一次的计算结果可以直接给下一步使用，一直迭代到temp 内部无数据
function calcCycle(cycles) {
    let res = [];
    let cycle_arr = cycles.toArray();
    let temp = cycle_arr.map(el => { return [el] });
    res.push(temp);
    while (temp.length > 0) {
        let temp2 = [];

        // 1. 假设temp 是所有n-n 不相交的环的集合
        temp.forEach(cys => {
            // 2. 获取其中一组环cys 内的所有点
            let nodes = cys.flat();
            // 3. 遍历cycle_arr 中的每一条回路cy
            cycle_arr.forEach(cy => {
                // 判断回路cy 中的某一个点是否在cys 上，如果不在，则符合条件
                let indpt = true;
                for (const node of cy) {
                    // forEach 循环无法中途跳到上一层
                    if (nodes.indexOf(node) > -1) {
                        indpt = false;
                        break;
                    }
                }
                if (indpt) {
                    // 创建新的(n+1)-(n+1) 不相交回路组，并压入临时存储数组
                    temp2.push(cys.concat([cy]).sort())
                }
            })
        })
        // 去重
        temp = Distinct(temp2);
        res.push(temp);
    }
    res.pop();
    return res;
}
