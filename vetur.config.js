// 为vetur 插件指定所需文件的起始位置
module.exports = {
    settings: {
        "vetur.useWorkspaceDependencies": true,
        "vetur.experimental.templateInterpolationService": true
    },
    // **optional** default: `[{ root: './' }]`
    // support monorepos
    projects: [
        './web_src', // 指定项目根目录的简写
    ]
}