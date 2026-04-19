package com.example.bookmarkbackend.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.example.bookmarkbackend.entity.Bookmark;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface BookmarkMapper extends BaseMapper<Bookmark> {
}