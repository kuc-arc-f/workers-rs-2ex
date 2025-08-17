import React, { useState, useEffect } from 'react';

function PlanDialog({ plan, onClose, onSave }) {
  // フォームの入力データを管理するState
  const [formData, setFormData] = useState({ date: '', content: '' });

  // ダイアログが開かれたとき、または編集対象のplanが変わったときにフォームの初期値を設定
  useEffect(() => {
    if (plan) {
      setFormData({
        date: plan.date || '',
        content: plan.content || '',
      });
    }
  }, [plan]);

  // 入力値が変更されたときにStateを更新
  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  // 保存ボタンが押されたときの処理
  const handleSubmit = (e) => {
    e.preventDefault();
    onSave({ ...plan, ...formData }); // 元のplanデータとフォームデータをマージして保存
  };

  if (!plan) return null; // planがなければ何も表示しない

  return (
    // オーバーレイ
    <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      {/* ダイアログ本体 */}
      <div className="bg-white p-6 rounded-lg shadow-xl w-full max-w-md">
        <h2 className="text-xl font-bold mb-4">
          {plan.id ? 'Edit Plan' : 'Create Plan'}
        </h2>
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="plan_date" className="block text-sm font-medium text-gray-700 mb-1">
              Date
            </label>
            <input
              type="date"
              id="plan_date"
              name="date"
              disabled={plan.id ? true : false}
              value={formData.date}
              onChange={handleChange}
              className="w-full p-2 border border-gray-300 rounded-md"
              required
            />
          </div>
          <div className="mb-6">
            <label htmlFor="content" className="block text-sm font-medium text-gray-700 mb-1">
              Content
            </label>
            <textarea
              id="content"
              name="content"
              rows="4"
              value={formData.content}
              onChange={handleChange}
              className="w-full p-2 border border-gray-300 rounded-md"
              required
            ></textarea>
          </div>
          <div className="flex justify-end space-x-2">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              Save
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default PlanDialog;