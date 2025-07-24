// src/components/ItemModal.js
import React, { useState, useEffect } from 'react';

// 新規作成時のフォームの初期状態
const getInitialFormData = () => ({
  title: '',
  description: '',
  completed: false,
  content_type: '',
  is_public: false, // false: 非公開, true: 公開
  food_orange: false,
  food_apple: false,
  food_banana: false,
  food_melon: false,
  food_grape: false,
  pub_date1: '',
  pub_date2: '',
  pub_date3: '',
  pub_date4: '',
  pub_date5: '',
  qty1: '',
  qty2: '',
  qty3: '',
  qty4: '',
  qty5: '',
  qty6: '',
});

function ItemModal({ item, onClose, onSave }) {
  const [formData, setFormData] = useState(getInitialFormData());
  const [errors, setErrors] = useState({});

  useEffect(() => {
    // 編集モードの場合、渡された item のデータでフォームを初期化
    if (item) {
      setFormData({ ...getInitialFormData(), ...item });
    } else {
      // 新規作成モードの場合、フォームを空にする
      setFormData(getInitialFormData());
    }
  }, [item]);

  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    let finalValue;

    if (type === 'checkbox') {
        finalValue = checked;
    } else if (type === 'radio') {
        // is_public は 'true'/'false' の文字列で来るので boolean に変換
        finalValue = value === 'true';
    } else {
        finalValue = value;
    }

    setFormData(prev => ({ ...prev, [name]: finalValue }));
  };

  const validate = () => {
    const newErrors = {};
    if (!formData.title.trim()) {
      newErrors.title = 'タイトルは必須項目です。';
    }
    // 他のバリデーションルールもここに追加可能
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (validate()) {
      onSave(formData);
    }
  };
  
  const foods = [
    { name: 'food_orange', label: 'オレンジ' },
    { name: 'food_apple', label: 'りんご' },
    { name: 'food_banana', label: 'バナナ' },
    { name: 'food_melon', label: 'メロン' },
    { name: 'food_grape', label: 'ぶどう' },
  ];

  const pubDates = [1, 2, 3, 4, 5];
  const quantities = [1, 2, 3, 4, 5, 6];

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex justify-center items-start pt-10 pb-10 overflow-y-auto">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl transform transition-all">
        <form onSubmit={handleSubmit}>
          <div className="px-6 py-4">
            <div className="flex justify-between items-center pb-3 border-b">
              <h3 className="text-xl font-semibold text-gray-800">{item ? 'アイテムの編集' : 'アイテムの新規追加'}</h3>
              <button type="button" onClick={onClose} className="text-gray-400 hover:text-gray-600">×</button>
            </div>
            
            {/* フォーム本体 */}
            <div className="mt-4 space-y-4 max-h-[70vh] overflow-y-auto pr-2">
              {/* Title */}
              <div>
                <label htmlFor="title" className="block text-sm font-medium text-gray-700">タイトル *</label>
                <input type="text" name="title" id="title" value={formData.title} onChange={handleChange} className={`mt-1 block w-full border rounded-md shadow-sm p-2 ${errors.title ? 'border-red-500' : 'border-gray-300'}`} />
                {errors.title && <p className="text-red-500 text-xs mt-1">{errors.title}</p>}
              </div>

              {/* Description */}
              <div>
                <label htmlFor="description" className="block text-sm font-medium text-gray-700">説明</label>
                <input type="text" name="description" id="description" value={formData.description} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
              </div>
              
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Completed */}
                <div className="flex items-center">
                    <input type="checkbox" name="completed" id="completed" checked={formData.completed} onChange={handleChange} className="h-4 w-4 text-blue-600 border-gray-300 rounded" />
                    <label htmlFor="completed" className="ml-2 block text-sm text-gray-900">完了</label>
                </div>

                {/* Content Type */}
                <div>
                  <label htmlFor="content_type" className="block text-sm font-medium text-gray-700">コンテンツタイプ</label>
                  <input type="text" name="content_type" id="content_type" value={formData.content_type} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
                </div>
              </div>
              
              {/* Is Public */}
              <div>
                <span className="block text-sm font-medium text-gray-700">公開設定</span>
                <div className="mt-2 space-x-4">
                  <label className="inline-flex items-center">
                    <input type="radio" name="is_public" value="true" checked={formData.is_public === true} onChange={handleChange} className="form-radio" />
                    <span className="ml-2">公開</span>
                  </label>
                  <label className="inline-flex items-center">
                    <input type="radio" name="is_public" value="false" checked={formData.is_public === false} onChange={handleChange} className="form-radio" />
                    <span className="ml-2">非公開</span>
                  </label>
                </div>
              </div>

              {/* Foods */}
              <div>
                <span className="block text-sm font-medium text-gray-700">好きな食べ物</span>
                <div className="mt-2 grid grid-cols-2 sm:grid-cols-3 gap-2">
                  {foods.map(food => (
                    <label key={food.name} className="inline-flex items-center">
                      <input type="checkbox" name={food.name} checked={!!formData[food.name]} onChange={handleChange} className="form-checkbox" />
                      <span className="ml-2">{food.label}</span>
                    </label>
                  ))}
                </div>
              </div>

              {/* Pub Dates & Quantities */}
              <div className="space-y-2">
                <span className="block text-sm font-medium text-gray-700">公開日と数量</span>
                {pubDates.map(i => (
                  <div key={i} className="grid grid-cols-2 gap-4">
                    <input type="date" name={`pub_date${i}`} value={formData[`pub_date${i}`]} onChange={handleChange} className="block w-full border border-gray-300 rounded-md shadow-sm p-2" />
                    <input type="text" name={`qty${i}`} value={formData[`qty${i}`]} onChange={handleChange} placeholder={`数量 ${i}`} className="block w-full border border-gray-300 rounded-md shadow-sm p-2" />
                  </div>
                ))}
                {/* qty6 */}
                 <div className="grid grid-cols-2 gap-4">
                    <div></div>
                    <input type="text" name="qty6" value={formData.qty6} onChange={handleChange} placeholder="数量 6" className="block w-full border border-gray-300 rounded-md shadow-sm p-2" />
                  </div>
              </div>
            </div>
          </div>
          
          {/* フッター */}
          <div className="px-6 py-3 bg-gray-50 text-right">
            <button type="button" onClick={onClose} className="mr-2 bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded">
              キャンセル
            </button>
            <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
              保存
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default ItemModal;