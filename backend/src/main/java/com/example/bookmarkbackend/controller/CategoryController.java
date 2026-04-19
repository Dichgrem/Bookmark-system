package com.example.bookmarkbackend.controller;

import com.example.bookmarkbackend.entity.Category;
import com.example.bookmarkbackend.service.CategoryService;
import com.example.bookmarkbackend.util.Result;
import org.springframework.web.bind.annotation.*;

import javax.annotation.Resource;
import java.util.List;

@RestController
@RequestMapping("/category")
public class CategoryController {

    @Resource
    private CategoryService categoryService;

    @GetMapping("/list")
    public Result<List<Category>> list(Long userId) {
        List<Category> list = categoryService.lambdaQuery()
                .eq(Category::getUserId, userId)
                .list();
        return Result.success(list);
    }

    @PostMapping("/add")
    public Result<Category> add(@RequestBody Category category) {
        categoryService.saveOrUpdate(category);
        return Result.success(category);
    }

    @PostMapping("/delete")
    public Result delete(@RequestBody Category category) {
        categoryService.removeById(category.getId());
        return Result.success();
    }
}