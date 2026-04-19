package com.example.bookmarkbackend.controller;

import com.example.bookmarkbackend.entity.User;
import com.example.bookmarkbackend.service.UserService;
import com.example.bookmarkbackend.util.Result;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/user")
public class UserController {

    // 变量名小写（修复注入问题）
    @Autowired
    private UserService userService;

    @PostMapping("/login")
    public Result login(@RequestBody User user) {
        User one = userService.lambdaQuery()
                .eq(User::getUsername, user.getUsername())
                .eq(User::getPassword, user.getPassword())
                .one();

        if (one == null) {
            return Result.error("账号或密码错误");
        }
        return Result.success(one);
    }

    @PostMapping("/register")
    public Result register(@RequestBody User user) {
        userService.save(user);
        return Result.success("注册成功");
    }
}