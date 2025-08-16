// src/components/ItemForm.tsx

import React, { useState, useEffect } from 'react';
import { Item } from '../types';

interface ItemFormProps {
  initialData: Item;
  onSave: (item: Item) => void;
  onCancel: () => void;
}

const ItemForm: React.FC<ItemFormProps> = ({ initialData, onSave, onCancel }) => {
  const [formData, setFormData] = useState<Item>(initialData);
  const [errors, setErrors] = useState<{ title?: string }>({});

  useEffect(() => {
    setFormData(initialData);
  }, [initialData]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value
    }));
  };

  const validate = (): boolean => {
    const newErrors: { title?: string } = {};
    if (!formData.title.trim()) {
      newErrors.title = 'Titleは必須です。';
    }
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validate()) {
      onSave(formData);
    }
  };
  
  // フォーム項目をグループ化して管理しやすくする
  const textInputs1 = [
    { name: 'title', label: 'Title' },
    { name: 'content', label: 'Content' },
    { name: 'content_type', label: 'Content Type' },
  ];

  const addressInputs = [
    { name: 'country_jp', label: 'Country (JP)' }, { name: 'country_en', label: 'Country (EN)' },
    { name: 'prefecture_jp', label: 'Prefecture (JP)' }, { name: 'prefecture_en', label: 'Prefecture (EN)' },
    { name: 'post_no_jp', label: 'Post No (JP)' }, { name: 'post_no_en', label: 'Post No (EN)' },
    { name: 'address_1_jp', label: 'Address 1 (JP)' }, { name: 'address_1_en', label: 'Address 1 (EN)' },
    { name: 'address_2_jp', label: 'Address 2 (JP)' }, { name: 'address_2_en', label: 'Address 2 (EN)' },
    { name: 'address_other_jp', label: 'Address Other (JP)' }, { name: 'address_other_en', label: 'Address Other (EN)' },
  ];

  const dateAndQtyInputs = [
    { name: 'pub_date1', label: 'Pub Date 1' }, { name: 'qty1', label: 'Qty 1' },
    { name: 'pub_date2', label: 'Pub Date 2' }, { name: 'qty2', label: 'Qty 2' },
    { name: 'pub_date3', label: 'Pub Date 3' }, { name: 'qty3', label: 'Qty 3' },
    { name: 'pub_date4', label: 'Pub Date 4' }, { name: 'qty4', label: 'Qty 4' },
    { name: 'pub_date5', label: 'Pub Date 5' }, { name: 'qty5', label: 'Qty 5' },
    { name: 'pub_date6', label: 'Pub Date 6' }, { name: 'qty6', label: 'Qty 6' },
  ];

  const foodCheckboxes = [
    { name: 'food_orange', label: 'Orange' }, { name: 'food_apple', label: 'Apple' },
    { name: 'food_banana', label: 'Banana' }, { name: 'food_melon', label: 'Melon' },
    { name: 'food_grape', label: 'Grape' },
  ];

  const categoryCheckboxes = [
    { name: 'category_food', label: 'Food' }, { name: 'category_drink', label: 'Drink' },
    { name: 'category_gadget', label: 'Gadget' }, { name: 'category_sport', label: 'Sport' },
    { name: 'category_government', label: 'Government' }, { name: 'category_internet', label: 'Internet' },
    { name: 'category_smartphone', label: 'Smartphone' },
  ];

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      
      {/* 基本情報 */}
      <div className="grid grid-cols-1 gap-4">
        {textInputs1.map(input => (
          <div key={input.name}>
            <label className="block text-sm font-medium text-gray-700">{input.label}</label>
            <input
              type="text"
              name={input.name}
              value={formData[input.name as keyof Item] as string}
              onChange={handleChange}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
            />
            {input.name === 'title' && errors.title && <p className="text-red-500 text-xs mt-1">{errors.title}</p>}
          </div>
        ))}
      </div>

      {/* 公開設定 */}
      <div>
        <label className="block text-sm font-medium text-gray-700">Public Type</label>
        <div className="mt-2 flex space-x-4">
          <label className="inline-flex items-center">
            <input type="radio" name="public_type" value="public" checked={formData.public_type === 'public'} onChange={handleChange} className="form-radio" />
            <span className="ml-2">Public</span>
          </label>
          <label className="inline-flex items-center">
            <input type="radio" name="public_type" value="private" checked={formData.public_type === 'private'} onChange={handleChange} className="form-radio" />
            <span className="ml-2">Private</span>
          </label>
        </div>
      </div>
      
      {/* 食べ物 */}
      <fieldset className="border p-4 rounded-md">
        <legend className="text-sm font-medium text-gray-700 px-1">Foods</legend>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4 mt-2">
            {foodCheckboxes.map(food => (
              <label key={food.name} className="inline-flex items-center">
                <input type="checkbox" name={food.name} checked={formData[food.name as keyof Item] as boolean} onChange={handleChange} className="form-checkbox" />
                <span className="ml-2">{food.label}</span>
              </label>
            ))}
        </div>
      </fieldset>

      {/* カテゴリ */}
      <fieldset className="border p-4 rounded-md">
        <legend className="text-sm font-medium text-gray-700 px-1">Categories</legend>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4 mt-2">
            {categoryCheckboxes.map(cat => (
              <label key={cat.name} className="inline-flex items-center">
                <input type="checkbox" name={cat.name} checked={formData[cat.name as keyof Item] as boolean} onChange={handleChange} className="form-checkbox" />
                <span className="ml-2">{cat.label}</span>
              </label>
            ))}
        </div>
      </fieldset>

      {/* 住所 */}
      <fieldset className="border p-4 rounded-md">
        <legend className="text-sm font-medium text-gray-700 px-1">Address</legend>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-2">
          {addressInputs.map(input => (
            <div key={input.name}>
              <label className="block text-sm font-medium text-gray-700">{input.label}</label>
              <input
                type="text"
                name={input.name}
                value={formData[input.name as keyof Item] as string}
                onChange={handleChange}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              />
            </div>
          ))}
        </div>
      </fieldset>

      {/* 日付と数量 */}
      <fieldset className="border p-4 rounded-md">
        <legend className="text-sm font-medium text-gray-700 px-1">Dates & Quantities</legend>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-2">
          {dateAndQtyInputs.map(input => (
            <div key={input.name}>
              <label className="block text-sm font-medium text-gray-700">{input.label}</label>
              <input
                type={input.name.startsWith('pub_date') ? 'date' : 'text'}
                name={input.name}
                value={formData[input.name as keyof Item] as string}
                onChange={handleChange}
                className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              />
            </div>
          ))}
        </div>
      </fieldset>

      {/* ボタン */}
      <div className="flex justify-end space-x-4 pt-4 border-t">
        <button type="button" onClick={onCancel} className="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300">
          キャンセル
        </button>
        <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
          保存
        </button>
      </div>
    </form>
  );
};

export default ItemForm;