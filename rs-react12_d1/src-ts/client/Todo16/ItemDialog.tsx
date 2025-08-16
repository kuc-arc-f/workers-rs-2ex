import React, { useState, useEffect } from 'react';
import { Item, ItemData } from '../types';

// 空のフォームの初期状態を生成するヘルパー関数
const createDefaultItemData = (): ItemData => ({
  title: '',
  content: '',
  contentType: '',
  publicType: 'draft',
  foodOrange: false,
  foodApple: false,
  foodBanana: false,
  foodMelon: false,
  foodGrape: false,
  categoryFood: false,
  categoryDrink: false,
  categoryGadget: false,
  categorySport: false,
  categoryGovernment: false,
  categoryInternet: false,
  categorySmartphone: false,
  countryJp: '',
  countryEn: '',
  prefectureJp: '',
  prefectureEn: '',
  postNoJp: '',
  postNoEn: '',
  address1Jp: '',
  address1En: '',
  address2Jp: '',
  address2En: '',
  addressOtherJp: '',
  addressOtherEn: '',
  pubDate1: '',
  pubDate2: '',
  pubDate3: '',
  pubDate4: '',
  pubDate5: '',
  pubDate6: '',
  qty1: '',
  qty2: '',
  qty3: '',
  qty4: '',
  qty5: '',
  qty6: '',
});

interface ItemDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (item: ItemData) => void;
  itemToEdit?: Item | null;
}

// エラーの状態を管理するための型
type FormErrors = {
  [key in keyof ItemData]?: string;
};

export const ItemDialog: React.FC<ItemDialogProps> = ({ isOpen, onClose, onSave, itemToEdit }) => {
  const [formData, setFormData] = useState<ItemData>(createDefaultItemData());
  const [errors, setErrors] = useState<FormErrors>({});

  // itemToEditが変更されたらフォームのデータを更新
  useEffect(() => {
    if (itemToEdit) {
      setFormData(itemToEdit);
    } else {
      setFormData(createDefaultItemData());
    }
    // ダイアログが開くときにエラーをリセット
    setErrors({});
  }, [itemToEdit, isOpen]);

  if (!isOpen) {
    return null;
  }
  
  // 入力値の変更をハンドルする汎用的な関数
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }));
  };

  // フォームのバリデーション
  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};
    if (!formData.title.trim()) {
      newErrors.title = 'Title は必須項目です。';
    }
    // 他のバリデーションルールもここに追加できます
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  // 保存ボタンがクリックされたときの処理
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validateForm()) {
      onSave(formData);
    }
  };

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog-content" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h2>{itemToEdit ? 'アイテムを編集' : '新規アイテムを作成'}</h2>
          <button className="button" onClick={onClose}>&times;</button>
        </div>
        <form onSubmit={handleSubmit} className="dialog-form">
          {/* ----- 基本情報 ----- */}
          <div className="form-group form-group-full">
            <label htmlFor="title">Title</label>
            <input type="text" id="title" name="title" value={formData.title} onChange={handleChange} />
            {errors.title && <span className="error-message">{errors.title}</span>}
          </div>
          <div className="form-group form-group-full">
            <label htmlFor="content">Content</label>
            <input type="text" id="content" name="content" value={formData.content} onChange={handleChange} />
          </div>
          <div className="form-group">
            <label htmlFor="contentType">Content Type</label>
            <input type="text" id="contentType" name="contentType" value={formData.contentType} onChange={handleChange} />
          </div>

          {/* ----- 公開タイプ (Radio) ----- */}
          <div className="form-group">
            <label>Public Type</label>
            <div className="form-group-radio">
              <input type="radio" id="public" name="publicType" value="public" checked={formData.publicType === 'public'} onChange={handleChange} />
              <label htmlFor="public">Public</label>
            </div>
            <div className="form-group-radio">
              <input type="radio" id="private" name="publicType" value="private" checked={formData.publicType === 'private'} onChange={handleChange} />
              <label htmlFor="private">Private</label>
            </div>
             <div className="form-group-radio">
              <input type="radio" id="draft" name="publicType" value="draft" checked={formData.publicType === 'draft'} onChange={handleChange} />
              <label htmlFor="draft">Draft</label>
            </div>
          </div>
          
          {/* ----- 食べ物 (Checkbox) ----- */}
          <div className="form-group">
             <label>Foods</label>
             {['Orange', 'Apple', 'Banana', 'Melon', 'Grape'].map(food => (
                <div key={food} className="form-group-checkbox">
                  <input type="checkbox" id={`food${food}`} name={`food${food}`} checked={formData[`food${food}` as keyof ItemData] as boolean} onChange={handleChange} />
                  <label htmlFor={`food${food}`}>{food}</label>
                </div>
             ))}
          </div>

          {/* ----- カテゴリ (Checkbox) ----- */}
          <div className="form-group">
            <label>Categories</label>
            {['Food', 'Drink', 'Gadget', 'Sport', 'Government', 'Internet', 'Smartphone'].map(cat => (
              <div key={cat} className="form-group-checkbox">
                <input type="checkbox" id={`category${cat}`} name={`category${cat}`} checked={formData[`category${cat}` as keyof ItemData] as boolean} onChange={handleChange} />
                <label htmlFor={`category${cat}`}>{cat}</label>
              </div>
            ))}
          </div>

          {/* ----- 住所関連 (JP/EN) ----- */}
          {['country', 'prefecture', 'postNo', 'address1', 'address2', 'addressOther'].map(field => (
            <React.Fragment key={field}>
              <div className="form-group">
                <label htmlFor={`${field}Jp`}>{field} (JP)</label>
                <input type="text" id={`${field}Jp`} name={`${field}Jp`} value={formData[`${field}Jp` as keyof ItemData] as string} onChange={handleChange} />
              </div>
               <div className="form-group">
                <label htmlFor={`${field}En`}>{field} (EN)</label>
                <input type="text" id={`${field}En`} name={`${field}En`} value={formData[`${field}En` as keyof ItemData] as string} onChange={handleChange} />
              </div>
            </React.Fragment>
          ))}

          {/* ----- 日付と数量 ----- */}
          {[1, 2, 3, 4, 5, 6].map(num => (
            <React.Fragment key={num}>
              <div className="form-group">
                <label htmlFor={`pubDate${num}`}>Pub Date {num}</label>
                <input type="date" id={`pubDate${num}`} name={`pubDate${num}`} value={formData[`pubDate${num}` as keyof ItemData] as string} onChange={handleChange} />
              </div>
              <div className="form-group">
                <label htmlFor={`qty${num}`}>Qty {num}</label>
                <input type="text" id={`qty${num}`} name={`qty${num}`} value={formData[`qty${num}` as keyof ItemData] as string} onChange={handleChange} />
              </div>
            </React.Fragment>
          ))}

          <div className="dialog-actions">
            <button type="button" className="button button-secondary" onClick={onClose}>キャンセル</button>
            <button type="submit" className="button button-primary">保存</button>
          </div>
        </form>
      </div>
    </div>
  );
};