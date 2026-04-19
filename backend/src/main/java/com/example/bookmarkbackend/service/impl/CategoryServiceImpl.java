package com.example.bookmarkbackend.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.example.bookmarkbackend.entity.Category;
import com.example.bookmarkbackend.mapper.CategoryMapper;
import com.example.bookmarkbackend.service.CategoryService;
import org.springframework.stereotype.Service;

@Service
public class CategoryServiceImpl extends ServiceImpl<CategoryMapper, Category> implements CategoryService {
}