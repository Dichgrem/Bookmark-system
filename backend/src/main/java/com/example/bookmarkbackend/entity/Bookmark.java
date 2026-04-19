package com.example.bookmarkbackend.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

@Data
@TableName("bookmark")
public class Bookmark {
    @TableId(type = IdType.AUTO)
    private Long id;

    private String title;
    private String url;
    private Long categoryId;
    private Long userId;
}
