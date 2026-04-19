package com.example.bookmarkbackend.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.example.bookmarkbackend.entity.User;
import com.example.bookmarkbackend.mapper.UserMapper;
import com.example.bookmarkbackend.service.UserService;
import org.springframework.stereotype.Service;

@Service
public class UserServiceImpl extends ServiceImpl<UserMapper, User> implements UserService {
}
