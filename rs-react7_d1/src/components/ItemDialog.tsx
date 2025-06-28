import React, { useState, useEffect } from 'react';

// 新規アイテムの空のテンプレート
const getInitialData = () => ({
  id: null,
  title: '',
  description: '',
  completed: false,
  content_type: '',
  public_type: 'public', // デフォルト値
  food_orange: false,
  food_apple: false,
  food_banana: false,
  food_melon: false,
  food_grape: false,
  category_food: false,
  category_drink: false,
  category_gadget: false,
  category_sport: false,
  category_government: false,
  category_internet: false,
  category_smartphone: false,
  country_jp: '',
  country_en: '',
  prefecture_jp: '',
  prefecture_en: '',
});

const ItemDialog = ({ item, onSave, onClose }) => {
  const [formData, setFormData] = useState(getInitialData());
  const [error, setError] = useState('');

  // ダイアログが開かれたとき、編集対象のアイテムデータをフォームにセットする
  useEffect(() => {
    if (item) {
      setFormData(item);
    } else {
      setFormData(getInitialData());
    }
  }, [item]);

  // フォームの入力値をハンドルする汎用的な関数
  const handleChange = (e) => {
    const { name, value, type, checked } = e.target;
    const newValue = type === 'checkbox' ? checked : value;
    setFormData(prev => ({ ...prev, [name]: newValue }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    // titleの入力チェック
    if (!formData.title.trim()) {
      setError('タイトルは必須項目です。');
      return;
    }
    setError(''); // エラーがなければクリア
    onSave(formData);
  };

  return (
    // ダイアログの背景（オーバーレイ）
    <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      {/* ダイアログ本体 */}
      <div className="bg-white p-6 rounded-lg shadow-xl w-full max-w-3xl max-h-[90vh] overflow-y-auto">
        <h2 className="text-2xl font-bold mb-4">{item ? 'アイテムを編集' : 'アイテムを追加'}</h2>
        
        <form onSubmit={handleSubmit}>
          {/* 基本情報 */}
          <div className="mb-4">
            <label htmlFor="title" className="block text-sm font-medium text-gray-700">タイトル <span className="text-red-500">*</span></label>
            <input type="text" name="title" id="title" value={formData.title} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
            {error && <p className="text-red-500 text-sm mt-1">{error}</p>}
          </div>

          <div className="mb-4">
            <label htmlFor="description" className="block text-sm font-medium text-gray-700">説明</label>
            <input type="text" name="description" id="description" value={formData.description} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
          </div>

          {/* グリッドレイアウトで項目を整理 */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-4">
            <div>
              <label className="block text-sm font-medium text-gray-700">公開設定</label>
              <div className="mt-2 space-x-4">
                <label><input type="radio" name="public_type" value="public" checked={formData.public_type === 'public'} onChange={handleChange} /> 公開</label>
                <label><input type="radio" name="public_type" value="private" checked={formData.public_type === 'private'} onChange={handleChange} /> 非公開</label>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700">ステータス</label>
              <div className="mt-2">
                <label><input type="checkbox" name="completed" checked={formData.completed} onChange={handleChange} /> 完了</label>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700">好きな食べ物</label>
              <div className="mt-2 grid grid-cols-2 gap-2">
                <label><input type="checkbox" name="food_orange" checked={formData.food_orange} onChange={handleChange} /> オレンジ</label>
                <label><input type="checkbox" name="food_apple" checked={formData.food_apple} onChange={handleChange} /> りんご</label>
                <label><input type="checkbox" name="food_banana" checked={formData.food_banana} onChange={handleChange} /> バナナ</label>
                <label><input type="checkbox" name="food_melon" checked={formData.food_melon} onChange={handleChange} /> メロン</label>
                <label><input type="checkbox" name="food_grape" checked={formData.food_grape} onChange={handleChange} /> ぶどう</label>
              </div>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700">カテゴリ</label>
              <div className="mt-2 grid grid-cols-2 gap-2">
                <label><input type="checkbox" name="category_food" checked={formData.category_food} onChange={handleChange} /> 食品</label>
                <label><input type="checkbox" name="category_drink" checked={formData.category_drink} onChange={handleChange} /> 飲料</label>
                <label><input type="checkbox" name="category_gadget" checked={formData.category_gadget} onChange={handleChange} /> ガジェット</label>
                <label><input type="checkbox" name="category_sport" checked={formData.category_sport} onChange={handleChange} /> スポーツ</label>
                <label><input type="checkbox" name="category_government" checked={formData.category_government} onChange={handleChange} /> 行政</label>
                <label><input type="checkbox" name="category_internet" checked={formData.category_internet} onChange={handleChange} /> ネット</label>
                <label><input type="checkbox" name="category_smartphone" checked={formData.category_smartphone} onChange={handleChange} /> スマホ</label>
              </div>
            </div>

            <div>
              <label htmlFor="content_type" className="block text-sm font-medium text-gray-700">コンテンツタイプ</label>
              <input type="text" name="content_type" id="content_type" value={formData.content_type} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
            </div>
          </div>
          
          {/* 国・都道府県 */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-4">
              <div>
                <label htmlFor="country_jp" className="block text-sm font-medium text-gray-700">国 (JP)</label>
                <input type="text" name="country_jp" id="country_jp" value={formData.country_jp} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
              </div>
              <div>
                <label htmlFor="country_en" className="block text-sm font-medium text-gray-700">国 (EN)</label>
                <input type="text" name="country_en" id="country_en" value={formData.country_en} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
              </div>
              <div>
                <label htmlFor="prefecture_jp" className="block text-sm font-medium text-gray-700">都道府県 (JP)</label>
                <input type="text" name="prefecture_jp" id="prefecture_jp" value={formData.prefecture_jp} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
              </div>
              <div>
                <label htmlFor="prefecture_en" className="block text-sm font-medium text-gray-700">都道府県 (EN)</label>
                <input type="text" name="prefecture_en" id="prefecture_en" value={formData.prefecture_en} onChange={handleChange} className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2" />
              </div>
          </div>


          {/* ボタン */}
          <div className="flex justify-end space-x-4 mt-6">
            <button type="button" onClick={onClose} className="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300">
              キャンセル
            </button>
            <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
              保存
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default ItemDialog;