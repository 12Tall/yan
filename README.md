# 燕/yan
一次基于`http`协议的前后分离，以`vue3`作为前端、`rust`作为后端的尝试。  
项目名称源自唐代诗人刘禹锡的《乌衣巷》：`旧时王谢堂前燕 飞入寻常百姓家`。

1. 创建`rust` 项目  
2. 在项目内创建`vue3`项目：`yarn create vite web_src --template vue`  
3. 在`vite.config.js`中修改输出目录  
4. 使用`rocket`作为消息中间件，配置项为`Rocket.toml`  
