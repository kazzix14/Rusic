# frozen_string_literal: true

require_relative "jungru/version"

require "rutie"

::Rutie.new(:jungru).init "init_jungru", __dir__

class PitchClass
  def minor_scale
    [1, 3, 4, 6, 8, 9, 11]
  end
end

class Scale
  def initialize(root, pitches)
    @root = root
    @pitches = pitches
    @big_pitches = []
    (0...12).map { |i| @big_pitches.concat pitches.map { |i2| i2 + 12 * i} }
  end
  
  def self.minor(root)
    Scale.new(root, [1, 3, 4, 6, 8, 9, 11])
  end

  def chords
    @pitches.map.with_index { |n, i| Chord.new(self, i, [1, 3, 5]) }
  end
  
  def nth(n)
    @big_pitches[n]
  end
  
  def root
    @root
  end
end 

class Chord
  def initialize(scale, root, intervals)
    @scale = scale
    @root = root
    @intervals = intervals
  end
  
  def add(interval)
    @intervals.push interval

    self
  end

  def omit(interval)
    @intervals.delete interval

    self
  end
  
  def note
    pitches = @intervals.map { |i| @scale.nth(@root + i) }
    freqs = pitches.map { |p| @scale.root.eq12[p] }
    { f: freqs }
  end
  
  def notes
    pitches = @intervals.map { |i| @scale.nth(@root + i) }
    pitches.map { |p| { f:@scale.root.eq12[p] } }
  end
end

module Jungru
  class ::Track
    def symbols
      [*(:a..:z), *(:A..:Z), *(:'0'..:'9')].each.with_index do |s, i|
        self.symbol s.to_sym, yield(s, i)
      end
    end
  end

  module Support
    module NilClassEx
      def nil_op_float(*args)
        nil
      end
      
      alias_method :+, :nil_op_float
      alias_method :-, :nil_op_float
      alias_method :*, :nil_op_float
      alias_method :/, :nil_op_float
      alias_method :amp, :nil_op_float
      alias_method :adsr, :nil_op_float
    end

    module FloatEx
      # rustで置き換えたい
      def *(other)
        case other
        when nil
          nil
        else
          super
        end
      end

      def +(other)
        case other
        when nil
          nil
        else
          super
        end
      end

      def amp(v)
        self * v
      end

      def eq12
        (0...120).map { |v| self * 2.0 ** (v.to_f() / 12.0) }
      end

      def minor_scale
        ::Scale.new(self, [1, 3, 4, 6, 8, 9, 11])
      end
    end
    
    module ArrayEx
    end
    
    class ::Array
      prepend ::Jungru::Support::ArrayEx
    end

    class ::Float
      prepend ::Jungru::Support::FloatEx
    end

    class ::NilClass
      prepend ::Jungru::Support::NilClassEx
    end
    
    def alphanumeric
      [*(:a..:z), *(:A..:Z), *(:'0'..:'9')]
    end
    
    def adsr(a, d, s, s_amp, r, t, last: nil)
      if t < a
        t / a
      elsif t < a + d
        1 + (s_amp - 1) * (t-a) / d
      elsif t < a + d + s
        s_amp
      elsif t < a + d + s + r
        s_amp * (1 - (t - (a + d + s)) / r)
      else
        last
      end
    end
    
    #def sin(hz, t)
    #  ru_sin(hz.to_f, t)
    #end

    #def saw(hz, t)
    #  ru_saw(hz.to_f, t)
    #end

    #def sq(hz, t)
    #  ru_sq(hz.to_f, t)
    #end
  end
end