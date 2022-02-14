# frozen_string_literal: true

require "test_helper"

class TestJungru < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Jungru::VERSION
  end

  def test_jungruby
    ::Jungru::Jungru.new do
      track :hat0, inst: nil do
        assign_each do |_k, i|
          [{ vel: 100, tone: i }]
        end

        assign :a, [{ vel: 100, tone: 0 }]

        section :a0, 16, 1,
        "
         .a.b .a.c .a.b .a.d
         .a.b ...c .a.b ...d
        "
      end
    end
  end

  def test_instrument
    j = ::Jungru::Instrument.load("apples")
    assert_equal "neko", j.neko
  end

  def test_musical_time
    assert_equal 1.bar, ::Jungru::MusicalTime.new(1, 1)
    assert_equal 2.bar, ::Jungru::MusicalTime.new(2, 1)
    assert_equal 8.th, ::Jungru::MusicalTime.new(1, 8)
    assert_equal 16.th, ::Jungru::MusicalTime.new(1, 16)
    assert_equal (4.of 16.th), 4.th
    assert_equal (16.th * 4), 4.th
  end
end
